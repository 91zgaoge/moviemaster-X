//! VNFO (Video NFO) 文件服务 - JSON格式存储影片信息
//! 文件名格式: {视频文件名}.vnfo

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// VNFO文件数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VNFOData {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub rating: Option<f64>,
    pub genres: Vec<String>,
    pub countries: Vec<String>,
    pub directors: Vec<String>,
    pub actors: Vec<String>,
    pub imdb_id: Option<String>,
    pub tmdb_id: Option<i64>,
    pub poster_url: Option<String>,
    pub video_type: Option<String>,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub source: String, // 数据来源: "tmdb", "douban", "manual"
    pub updated_at: String,
}

/// 获取VNFO文件路径
pub fn get_vnfo_path(video_path: &Path) -> PathBuf {
    let file_stem = video_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let parent = video_path.parent().unwrap_or(Path::new(""));
    parent.join(format!("{}.vnfo", file_stem))
}

/// 检查VNFO文件是否存在
pub fn vnfo_exists(video_path: &Path) -> bool {
    let vnfo_path = get_vnfo_path(video_path);
    vnfo_path.exists()
}

/// 读取VNFO文件
pub fn read_vnfo(video_path: &Path) -> Result<VNFOData, String> {
    let vnfo_path = get_vnfo_path(video_path);
    if !vnfo_path.exists() {
        return Err("VNFO文件不存在".to_string());
    }

    let content = fs::read_to_string(&vnfo_path)
        .map_err(|e| format!("读取VNFO文件失败: {}", e))?;

    let data: VNFOData = serde_json::from_str(&content)
        .map_err(|e| format!("解析VNFO文件失败: {}", e))?;

    log::info!("成功读取VNFO文件: {:?}", vnfo_path);
    Ok(data)
}

/// 保存VNFO文件
pub fn save_vnfo(video_path: &Path, data: &VNFOData) -> Result<(), String> {
    let vnfo_path = get_vnfo_path(video_path);

    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("序列化VNFO数据失败: {}", e))?;

    fs::write(&vnfo_path, json)
        .map_err(|e| format!("写入VNFO文件失败: {}", e))?;

    log::info!("成功保存VNFO文件: {:?}", vnfo_path);
    Ok(())
}

/// 从VNFO数据更新Movie结构
use crate::models::Movie;

pub fn apply_vnfo_to_movie(movie: &mut Movie, vnfo: &VNFOData) {
    if let Some(ref title) = vnfo.title {
        movie.cnname = Some(title.clone());
    }
    if let Some(ref original) = vnfo.original_title {
        movie.cnoname = Some(original.clone());
    }
    if let Some(ref year) = vnfo.year {
        movie.year = Some(year.clone());
    }
    if let Some(ref plot) = vnfo.plot {
        movie.description = Some(plot.clone());
    }
    if let Some(rating) = vnfo.rating {
        movie.douban_rating = Some(rating);
    }
    if !vnfo.countries.is_empty() {
        movie.countries = Some(vnfo.countries.join(", "));
    }
    if let Some(ref imdb_id) = vnfo.imdb_id {
        movie.imdb_id = Some(imdb_id.clone());
    }
}

/// 从TMDB详情创建VNFO数据
use crate::commands::movie::TMDBMovieDetail;

pub fn vnfo_from_tmdb_detail(detail: &TMDBMovieDetail, video_type: &str) -> VNFOData {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    VNFOData {
        title: Some(detail.cn_title.clone().unwrap_or_else(|| detail.title.clone())),
        original_title: Some(detail.title.clone()),
        year: detail.year.clone(),
        plot: detail.overview.clone(),
        rating: Some(detail.vote_average),
        genres: detail.genres.clone(),
        countries: detail.countries.clone(),
        directors: Vec::new(), // TMDB API需要单独获取
        actors: Vec::new(), // TMDB API需要单独获取
        imdb_id: detail.imdb_id.clone(),
        tmdb_id: Some(detail.id),
        poster_url: detail.poster_url.clone(),
        video_type: Some(video_type.to_string()),
        season: None,
        episode: None,
        source: "tmdb".to_string(),
        updated_at: now,
    }
}

/// 从Movie创建VNFO数据
pub fn vnfo_from_movie(movie: &Movie) -> VNFOData {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    VNFOData {
        title: movie.cnname.clone(),
        original_title: movie.cnoname.clone(),
        year: movie.year.clone(),
        plot: movie.description.clone(),
        rating: movie.douban_rating.or(movie.imdb_rating),
        genres: Vec::new(),
        countries: movie.countries.as_ref()
            .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default(),
        directors: Vec::new(),
        actors: Vec::new(),
        imdb_id: movie.imdb_id.clone(),
        tmdb_id: None,
        poster_url: movie.poster_path.clone(),
        video_type: Some(movie.video_type.clone()),
        season: movie.season.clone(),
        episode: movie.episode.clone(),
        source: "manual".to_string(),
        updated_at: now,
    }
}

//! 导入导出服务 - 支持CSV和JSON格式

use crate::models::Movie;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 导出格式
#[derive(Debug, Clone)]
pub enum ExportFormat {
    CSV,
    JSON,
}

/// 影片导出数据
#[derive(Debug, Serialize, Deserialize)]
pub struct MovieExportData {
    pub id: i64,
    pub filename: String,
    pub path: String,
    pub cnname: Option<String>,
    pub cnoname: Option<String>,
    pub year: Option<String>,
    pub countries: Option<String>,
    pub description: Option<String>,
    pub douban_rating: Option<f64>,
    pub imdb_rating: Option<f64>,
    pub imdb_id: Option<String>,
    pub video_type: String,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub file_size: Option<i64>,
}

impl From<&Movie> for MovieExportData {
    fn from(movie: &Movie) -> Self {
        Self {
            id: movie.id,
            filename: movie.filename.clone(),
            path: movie.path.clone(),
            cnname: movie.cnname.clone(),
            cnoname: movie.cnoname.clone(),
            year: movie.year.clone(),
            countries: movie.countries.clone(),
            description: movie.description.clone(),
            douban_rating: movie.douban_rating,
            imdb_rating: movie.imdb_rating,
            imdb_id: movie.imdb_id.clone(),
            video_type: movie.video_type.clone(),
            season: movie.season.clone(),
            episode: movie.episode.clone(),
            file_size: movie.file_size,
        }
    }
}

/// 导出影片列表到JSON
pub fn export_to_json(movies: &[Movie], output_path: &Path) -> Result<(), String> {
    let export_data: Vec<MovieExportData> = movies.iter().map(MovieExportData::from).collect();
    
    let json = serde_json::to_string_pretty(&export_data)
        .map_err(|e| format!("JSON serialization failed: {}", e))?;
    
    std::fs::write(output_path, json)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

/// 导出影片列表到CSV
pub fn export_to_csv(movies: &[Movie], output_path: &Path) -> Result<(), String> {
    let mut writer = csv::Writer::from_path(output_path)
        .map_err(|e| format!("Failed to create CSV writer: {}", e))?;
    
    // 写入表头
    writer.write_record(&[
        "ID", "文件名", "路径", "中文名", "原名", "年份", "国家", "简介",
        "豆瓣评分", "IMDB评分", "IMDB ID", "类型", "季", "集", "文件大小"
    ]).map_err(|e| e.to_string())?;
    
    // 写入数据
    for movie in movies {
        writer.write_record(&[
            movie.id.to_string(),
            movie.filename.clone(),
            movie.path.clone(),
            movie.cnname.clone().unwrap_or_default(),
            movie.cnoname.clone().unwrap_or_default(),
            movie.year.clone().unwrap_or_default(),
            movie.countries.clone().unwrap_or_default(),
            movie.description.clone().unwrap_or_default(),
            movie.douban_rating.map(|r| r.to_string()).unwrap_or_default(),
            movie.imdb_rating.map(|r| r.to_string()).unwrap_or_default(),
            movie.imdb_id.clone().unwrap_or_default(),
            movie.video_type.clone(),
            movie.season.clone().unwrap_or_default(),
            movie.episode.clone().unwrap_or_default(),
            movie.file_size.map(|s| s.to_string()).unwrap_or_default(),
        ]).map_err(|e| e.to_string())?;
    }
    
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// 从JSON导入影片数据
pub fn import_from_json(json_content: &str) -> Result<Vec<MovieExportData>, String> {
    let data: Vec<MovieExportData> = serde_json::from_str(json_content)
        .map_err(|e| format!("JSON parsing failed: {}", e))?;
    
    Ok(data)
}

/// 从CSV导入影片数据
pub fn import_from_csv(csv_path: &Path) -> Result<Vec<MovieExportData>, String> {
    let mut reader = csv::Reader::from_path(csv_path)
        .map_err(|e| format!("Failed to open CSV: {}", e))?;
    
    let mut movies = Vec::new();
    
    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        
        let movie = MovieExportData {
            id: record.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            filename: record.get(1).unwrap_or("").to_string(),
            path: record.get(2).unwrap_or("").to_string(),
            cnname: record.get(3).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            cnoname: record.get(4).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            year: record.get(5).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            countries: record.get(6).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            description: record.get(7).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            douban_rating: record.get(8).and_then(|s| s.parse().ok()),
            imdb_rating: record.get(9).and_then(|s| s.parse().ok()),
            imdb_id: record.get(10).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            video_type: record.get(11).unwrap_or("movie").to_string(),
            season: record.get(12).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            episode: record.get(13).filter(|s| !s.is_empty()).map(|s| s.to_string()),
            file_size: record.get(14).and_then(|s| s.parse().ok()),
        };
        
        movies.push(movie);
    }
    
    Ok(movies)
}
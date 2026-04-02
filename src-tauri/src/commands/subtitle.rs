//! 字幕下载命令

use crate::db::Database;
use crate::services::subtitle::SubtitleDownloader;
use tauri::State;
use std::path::Path;

/// 搜索字幕
#[tauri::command]
pub async fn search_subtitles(
    db: State<'_, Database>,
    movie_id: i64,
    language: String,
) -> Result<Vec<SubtitleResult>, String> {
    log::info!("Searching subtitles for movie {} in language {}", movie_id, language);
    
    // 获取影片信息
    let (imdb_id, file_hash, file_size) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        
        let mut stmt = conn.prepare(
            "SELECT imdb_id, file_hash, file_size FROM movies WHERE id = ?1"
        ).map_err(|e| e.to_string())?;
        
        stmt.query_row([movie_id], |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<i64>>(2)?,
            ))
        }).map_err(|e| e.to_string())?
    };
    
    // 这里简化处理，实际应该调用字幕API
    // 返回模拟数据用于测试
    let results = vec![
        SubtitleResult {
            id: "sub1".to_string(),
            filename: "movie_chs.srt".to_string(),
            language: language.clone(),
            format: "srt".to_string(),
            download_count: 1000,
            source: "OpenSubtitles".to_string(),
        },
        SubtitleResult {
            id: "sub2".to_string(),
            filename: "movie_cht.srt".to_string(),
            language: language.clone(),
            format: "srt".to_string(),
            download_count: 800,
            source: "OpenSubtitles".to_string(),
        },
    ];
    
    log::info!("Found {} subtitles for movie {}", results.len(), movie_id);
    Ok(results)
}

/// 下载字幕
#[tauri::command]
pub async fn download_subtitle(
    db: State<'_, Database>,
    movie_id: i64,
    subtitle_id: String,
    save_path: String,
) -> Result<String, String> {
    log::info!("Downloading subtitle {} for movie {} to {}", subtitle_id, movie_id, save_path);
    
    // 获取影片路径
    let movie_path: String = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        
        conn.query_row(
            "SELECT path FROM movies WHERE id = ?1",
            [movie_id],
            |row| row.get(0)
        ).map_err(|e| e.to_string())?
    };
    
    // 构建字幕保存路径（与影片同目录，同名不同扩展名）
    let movie_path_obj = Path::new(&movie_path);
    let movie_stem = movie_path_obj.file_stem()
        .ok_or("Invalid movie path")?
        .to_string_lossy();
    
    let subtitle_filename = format!("{}.srt", movie_stem);
    let subtitle_path = movie_path_obj.parent()
        .ok_or("Invalid movie directory")?
        .join(&subtitle_filename);
    
    // 这里应该调用实际的下载逻辑
    // 简化处理：创建一个空文件表示已下载
    std::fs::write(&subtitle_path, "1\n00:00:00,000 --> 00:00:05,000\nTest Subtitle\n")
        .map_err(|e| format!("Failed to save subtitle: {}", e))?;
    
    let subtitle_path_str = subtitle_path.to_string_lossy().to_string();
    
    // 更新数据库
    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE movies SET subtitle_path = ?1 WHERE id = ?2",
            [&subtitle_path_str, &movie_id.to_string()],
        ).map_err(|e| e.to_string())?;
    }
    
    log::info!("Subtitle downloaded to {}", subtitle_path_str);
    Ok(subtitle_path_str)
}

/// 字幕搜索结果
#[derive(Debug, serde::Serialize)]
pub struct SubtitleResult {
    pub id: String,
    pub filename: String,
    pub language: String,
    pub format: String,
    pub download_count: i32,
    pub source: String,
}
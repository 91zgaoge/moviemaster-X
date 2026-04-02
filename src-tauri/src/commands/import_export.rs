//! 导入导出命令

use crate::db::Database;
use crate::services::import_export::{export_to_csv, export_to_json, import_from_csv, import_from_json, MovieExportData};
use tauri::State;
use std::path::Path;

/// 导出影片列表到JSON
#[tauri::command]
pub fn export_movies_json(
    db: State<Database>,
    output_path: String,
) -> Result<(), String> {
    log::info!("Exporting movies to JSON: {}", output_path);
    
    let movies = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, directory_id, filename, path, cnname, cnoname, year, countries, 
             douban_id, imdb_id, poster_path, fanart_path, description, douban_rating, 
             imdb_rating, video_type, season, episode, file_size, file_hash, created_at, updated_at 
             FROM movies"
        ).map_err(|e| e.to_string())?;
        
        let movies = stmt.query_map([], |row| {
            Ok(crate::models::Movie {
                id: row.get(0)?,
                directory_id: row.get(1)?,
                filename: row.get(2)?,
                path: row.get(3)?,
                cnname: row.get(4)?,
                cnoname: row.get(5)?,
                year: row.get(6)?,
                countries: row.get(7)?,
                douban_id: row.get(8)?,
                imdb_id: row.get(9)?,
                poster_path: row.get(10)?,
                fanart_path: row.get(11)?,
                description: row.get(12)?,
                douban_rating: row.get(13)?,
                imdb_rating: row.get(14)?,
                video_type: row.get(15)?,
                season: row.get(16)?,
                episode: row.get(17)?,
                file_size: row.get(18)?,
                file_hash: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
        
        movies
    };
    
    export_to_json(&movies, Path::new(&output_path))
        .map_err(|e| format!("Export failed: {}", e))?;
    
    log::info!("Exported {} movies to {}", movies.len(), output_path);
    Ok(())
}

/// 导出影片列表到CSV
#[tauri::command]
pub fn export_movies_csv(
    db: State<Database>,
    output_path: String,
) -> Result<(), String> {
    log::info!("Exporting movies to CSV: {}", output_path);
    
    let movies = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, directory_id, filename, path, cnname, cnoname, year, countries, 
             douban_id, imdb_id, poster_path, fanart_path, description, douban_rating, 
             imdb_rating, video_type, season, episode, file_size, file_hash, created_at, updated_at 
             FROM movies"
        ).map_err(|e| e.to_string())?;
        
        let movies = stmt.query_map([], |row| {
            Ok(crate::models::Movie {
                id: row.get(0)?,
                directory_id: row.get(1)?,
                filename: row.get(2)?,
                path: row.get(3)?,
                cnname: row.get(4)?,
                cnoname: row.get(5)?,
                year: row.get(6)?,
                countries: row.get(7)?,
                douban_id: row.get(8)?,
                imdb_id: row.get(9)?,
                poster_path: row.get(10)?,
                fanart_path: row.get(11)?,
                description: row.get(12)?,
                douban_rating: row.get(13)?,
                imdb_rating: row.get(14)?,
                video_type: row.get(15)?,
                season: row.get(16)?,
                episode: row.get(17)?,
                file_size: row.get(18)?,
                file_hash: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
        
        movies
    };
    
    export_to_csv(&movies, Path::new(&output_path))
        .map_err(|e| format!("Export failed: {}", e))?;
    
    log::info!("Exported {} movies to {}", movies.len(), output_path);
    Ok(())
}

/// 从JSON导入影片数据
#[tauri::command]
pub fn import_movies_json(
    db: State<Database>,
    json_content: String,
) -> Result<usize, String> {
    log::info!("Importing movies from JSON");
    
    let import_data = import_from_json(&json_content)
        .map_err(|e| format!("Import parse failed: {}", e))?;
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut count = 0;
    
    for data in import_data {
        // 检查是否已存在相同路径的影片
        let exists: bool = conn.query_row(
            "SELECT 1 FROM movies WHERE path = ?1 LIMIT 1",
            [&data.path],
            |_| Ok(true)
        ).unwrap_or(false);
        
        if !exists {
            conn.execute(
                "INSERT INTO movies (filename, path, cnname, cnoname, year, countries, 
                 description, douban_rating, imdb_rating, imdb_id, video_type, season, episode, 
                 file_size, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, datetime('now'), datetime('now'))",
                [
                    &data.filename,
                    &data.path,
                    &data.cnname.unwrap_or_default(),
                    &data.cnoname.unwrap_or_default(),
                    &data.year.unwrap_or_default(),
                    &data.countries.unwrap_or_default(),
                    &data.description.unwrap_or_default(),
                    &data.douban_rating.map(|r| r.to_string()).unwrap_or_default(),
                    &data.imdb_rating.map(|r| r.to_string()).unwrap_or_default(),
                    &data.imdb_id.unwrap_or_default(),
                    &data.video_type,
                    &data.season.unwrap_or_default(),
                    &data.episode.unwrap_or_default(),
                    &data.file_size.map(|s| s.to_string()).unwrap_or_default(),
                ],
            ).map_err(|e| e.to_string())?;
            
            count += 1;
        }
    }
    
    log::info!("Imported {} movies from JSON", count);
    Ok(count)
}

/// 从CSV导入影片数据
#[tauri::command]
pub fn import_movies_csv(
    db: State<Database>,
    csv_path: String,
) -> Result<usize, String> {
    log::info!("Importing movies from CSV: {}", csv_path);
    
    let import_data = import_from_csv(Path::new(&csv_path))
        .map_err(|e| format!("Import parse failed: {}", e))?;
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut count = 0;
    
    for data in import_data {
        // 检查是否已存在相同路径的影片
        let exists: bool = conn.query_row(
            "SELECT 1 FROM movies WHERE path = ?1 LIMIT 1",
            [&data.path],
            |_| Ok(true)
        ).unwrap_or(false);
        
        if !exists {
            conn.execute(
                "INSERT INTO movies (filename, path, cnname, cnoname, year, countries, 
                 description, douban_rating, imdb_rating, imdb_id, video_type, season, episode, 
                 file_size, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, datetime('now'), datetime('now'))",
                [
                    &data.filename,
                    &data.path,
                    &data.cnname.unwrap_or_default(),
                    &data.cnoname.unwrap_or_default(),
                    &data.year.unwrap_or_default(),
                    &data.countries.unwrap_or_default(),
                    &data.description.unwrap_or_default(),
                    &data.douban_rating.map(|r| r.to_string()).unwrap_or_default(),
                    &data.imdb_rating.map(|r| r.to_string()).unwrap_or_default(),
                    &data.imdb_id.unwrap_or_default(),
                    &data.video_type,
                    &data.season.unwrap_or_default(),
                    &data.episode.unwrap_or_default(),
                    &data.file_size.map(|s| s.to_string()).unwrap_or_default(),
                ],
            ).map_err(|e| e.to_string())?;
            
            count += 1;
        }
    }
    
    log::info!("Imported {} movies from CSV", count);
    Ok(count)
}
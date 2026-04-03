use crate::db::Database;
use crate::models::Movie;
use crate::scanner;
use crate::services;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{Manager, State};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct DoubanSearchResult {
    pub id: String,
    pub title: String,
    pub year: String,
    pub rating: Option<f64>,
    pub poster: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoubanDetailResult {
    pub id: String,
    pub title: String,
    pub original_title: Option<String>,
    pub year: String,
    pub rating: Option<f64>,
    pub genres: Vec<String>,
    pub directors: Vec<String>,
    pub casts: Vec<String>,
    pub summary: Option<String>,
    pub countries: Vec<String>,
    pub poster: Option<String>,
}

#[tauri::command]
pub fn get_movies(
    db: State<Database>,
    directory_id: Option<i64>,
    video_type: Option<String>,
    search: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Movie>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut sql = String::from(
        "SELECT id, directory_id, filename, path, cnname, cnoname, year, countries, 
         douban_id, imdb_id, poster_path, fanart_path, description, douban_rating, 
         imdb_rating, video_type, season, episode, file_size, file_hash, 
         created_at, updated_at 
         FROM movies WHERE 1=1"
    );
    
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(dir_id) = directory_id {
        sql.push_str(" AND directory_id = ?");
        params_vec.push(Box::new(dir_id));
    }
    
    if let Some(vtype) = video_type {
        sql.push_str(" AND video_type = ?");
        params_vec.push(Box::new(vtype));
    }
    
    if let Some(s) = search {
        sql.push_str(" AND (cnname LIKE ? OR cnoname LIKE ? OR filename LIKE ?)");
        let search_pattern = format!("%{}%", s);
        params_vec.push(Box::new(search_pattern.clone()));
        params_vec.push(Box::new(search_pattern.clone()));
        params_vec.push(Box::new(search_pattern));
    }
    
    sql.push_str(" ORDER BY cnname, filename");
    
    if let Some(lim) = limit {
        sql.push_str(&format!(" LIMIT {}", lim));
    }
    
    if let Some(off) = offset {
        sql.push_str(&format!(" OFFSET {}", off));
    }
    
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    
    let movies = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(Movie {
                id: row.get(0)?,
                directory_id: row.get(1)?,
                series_id: row.get(2)?,
                filename: row.get(3)?,
                path: row.get(4)?,
                cnname: row.get(5)?,
                cnoname: row.get(6)?,
                year: row.get(7)?,
                countries: row.get(8)?,
                douban_id: row.get(9)?,
                imdb_id: row.get(10)?,
                poster_path: row.get(11)?,
                fanart_path: row.get(12)?,
                description: row.get(13)?,
                douban_rating: row.get(14)?,
                imdb_rating: row.get(15)?,
                video_type: row.get(16)?,
                season: row.get(17)?,
                episode: row.get(18)?,
                file_size: row.get(19)?,
                file_hash: row.get(20)?,
                created_at: row.get(21)?,
                updated_at: row.get(22)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    
    Ok(movies)
}

#[tauri::command]
pub async fn scan_directory(db: State<'_, Database>, directory_id: i64) -> Result<i32, String> {
    log::info!("Starting scan for directory_id: {}", directory_id);
    
    let (path, path_type) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT path, path_type FROM directories WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        
        let result: Result<(String, String), _> = stmt.query_row([directory_id], |row| {
            Ok((row.get(0)?, row.get::<_, String>(1).unwrap_or_else(|_| "local".to_string())))
        });
        
        match result {
            Ok(r) => r,
            Err(_) => return Err("Directory not found".to_string()),
        }
    };
    
    if path_type != "local" {
        return Err("SMB scanning not yet implemented".to_string());
    }
    
    let path_ref = Path::new(&path);
    if !path_ref.exists() {
        return Err("Directory does not exist".to_string());
    }
    
    let mut count = 0;
    
    for entry in WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        
        if entry_path.is_file() && scanner::is_video_file(entry_path) {
            let filename = entry_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            
            let file_path = entry_path.to_string_lossy().to_string();
            let file_size = entry.metadata().map(|m| m.len() as i64).ok();
            
            let parsed = services::parse_filename(&filename);
            
            // Calculate hash for smaller files (hash calculation is expensive)
            let file_hash = if file_size.unwrap_or(0) < 1024 * 1024 * 1024 { // < 1GB
                crate::services::hash::calculate_opensubtitles_hash(entry_path).ok()
            } else {
                None
            };
            
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            
            // Insert or ignore if already exists
            conn.execute(
                "INSERT OR IGNORE INTO movies 
                 (directory_id, filename, path, cnname, year, video_type, season, episode, file_size, file_hash) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    directory_id,
                    filename,
                    file_path,
                    parsed.title,
                    parsed.year,
                    parsed.video_type,
                    parsed.season,
                    parsed.episode,
                    file_size,
                    file_hash,
                ],
            ).map_err(|e| e.to_string())?;
            
            if conn.changes() > 0 {
                count += 1;
            }
        }
    }
    
    log::info!("Scan completed. Added {} new movies.", count);
    Ok(count)
}

#[tauri::command]
pub fn get_movie_by_id(db: State<Database>, id: i64) -> Result<Option<Movie>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare(
            "SELECT id, directory_id, filename, path, cnname, cnoname, year, countries, 
             douban_id, imdb_id, poster_path, fanart_path, description, douban_rating, 
             imdb_rating, video_type, season, episode, file_size, file_hash, 
             created_at, updated_at 
             FROM movies WHERE id = ?1"
        )
        .map_err(|e| e.to_string())?;
    
    let movie = stmt
        .query_row([id], |row| {
            Ok(Movie {
                id: row.get(0)?,
                directory_id: row.get(1)?,
                series_id: row.get(2)?,
                filename: row.get(3)?,
                path: row.get(4)?,
                cnname: row.get(5)?,
                cnoname: row.get(6)?,
                year: row.get(7)?,
                countries: row.get(8)?,
                douban_id: row.get(9)?,
                imdb_id: row.get(10)?,
                poster_path: row.get(11)?,
                fanart_path: row.get(12)?,
                description: row.get(13)?,
                douban_rating: row.get(14)?,
                imdb_rating: row.get(15)?,
                video_type: row.get(16)?,
                season: row.get(17)?,
                episode: row.get(18)?,
                file_size: row.get(19)?,
                file_hash: row.get(20)?,
                created_at: row.get(21)?,
                updated_at: row.get(22)?,
            })
        })
        .ok();
    
    Ok(movie)
}

#[tauri::command]
pub fn update_movie_info(
    db: State<Database>,
    id: i64,
    cnname: Option<String>,
    cnoname: Option<String>,
    year: Option<String>,
    countries: Option<String>,
    douban_id: Option<String>,
    imdb_id: Option<String>,
    description: Option<String>,
    douban_rating: Option<f64>,
    imdb_rating: Option<f64>,
    poster_path: Option<String>,
    fanart_path: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE movies SET 
         cnname = COALESCE(?1, cnname),
         cnoname = COALESCE(?2, cnoname),
         year = COALESCE(?3, year),
         countries = COALESCE(?4, countries),
         douban_id = COALESCE(?5, douban_id),
         imdb_id = COALESCE(?6, imdb_id),
         description = COALESCE(?7, description),
         douban_rating = COALESCE(?8, douban_rating),
         imdb_rating = COALESCE(?9, imdb_rating),
         poster_path = COALESCE(?10, poster_path),
         fanart_path = COALESCE(?11, fanart_path),
         updated_at = datetime('now')
         WHERE id = ?12",
        params![
            cnname, cnoname, year, countries, douban_id, imdb_id,
            description, douban_rating, imdb_rating, poster_path, fanart_path, id
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_douban(title: String, year: Option<String>) -> Result<Vec<DoubanSearchResult>, String> {
    log::info!("Searching Douban for: {} ({:?})", title, year);
    
    // Note: This is a placeholder implementation
    // In production, you would use a proper Douban API or web scraping
    // For now, return empty results - will be implemented with actual API
    
    // Try to use a free API alternative
    let client = reqwest::Client::new();
    
    // Using a public movie API as fallback (TMDB requires API key)
    let search_url = format!(
        "https://v2.sg.media-api.com/search?query={}&limit=10",
        urlencoding::encode(&title)
    );
    
    if let Ok(response) = client.get(&search_url).send().await {
        if let Ok(data) = response.json::<serde_json::Value>().await {
            let mut results = Vec::new();
            
            if let Some(results_array) = data.get("results").and_then(|r| r.as_array()) {
                for item in results_array {
                    let title_str = item.get("title")
                        .and_then(|t| t.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    // Simple title match
                    if title_str.to_lowercase().contains(&title.to_lowercase()) {
                        results.push(DoubanSearchResult {
                            id: item.get("id")
                                .and_then(|i| i.as_str())
                                .unwrap_or("")
                                .to_string(),
                            title: title_str,
                            year: item.get("release_date")
                                .and_then(|d| d.as_str())
                                .unwrap_or("")
                                .chars()
                                .take(4)
                                .collect(),
                            rating: item.get("vote_average")
                                .and_then(|r| r.as_f64())
                                .map(|r| r / 10.0 * 5.0), // Convert to 5-star rating
                            poster: item.get("poster_path")
                                .and_then(|p| p.as_str())
                                .map(|p| format!("https://image.tmdb.org/t/p/w200{}", p)),
                            genres: item.get("genre_ids")
                                .and_then(|g| g.as_array())
                                .map(|arr| arr.iter()
                                    .filter_map(|g| g.as_i64())
                                    .map(|g| g.to_string())
                                    .collect())
                                .unwrap_or_default(),
                        });
                    }
                }
            }
            
            if !results.is_empty() {
                return Ok(results);
            }
        }
    }
    
    // Return empty if no results
    Ok(Vec::new())
}

#[tauri::command]
pub async fn fetch_douban_info(db: State<'_, Database>, movie_id: i64) -> Result<Movie, String> {
    // First get the movie
    let movie = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT douban_id, cnname FROM movies WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        
        let result: Result<(String, String), _> = stmt.query_row([movie_id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        });
        
        match result {
            Ok(r) => r,
            Err(_) => return Err("Movie not found".to_string()),
        }
    };
    
    // Placeholder: In production, fetch actual Douban info
    // For now, return the existing movie
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, directory_id, filename, path, cnname, cnoname, year, countries, 
         douban_id, imdb_id, poster_path, fanart_path, description, douban_rating, 
         imdb_rating, video_type, season, episode, file_size, file_hash, 
         created_at, updated_at 
         FROM movies WHERE id = ?1"
    ).map_err(|e| e.to_string())?;
    
    let movie_result = stmt.query_row([movie_id], |row| {
        Ok(Movie {
            id: row.get(0)?,
            directory_id: row.get(1)?,
                series_id: row.get(2)?,
            filename: row.get(3)?,
            path: row.get(4)?,
            cnname: row.get(5)?,
            cnoname: row.get(6)?,
            year: row.get(7)?,
            countries: row.get(8)?,
            douban_id: row.get(9)?,
            imdb_id: row.get(10)?,
            poster_path: row.get(11)?,
            fanart_path: row.get(12)?,
            description: row.get(13)?,
            douban_rating: row.get(14)?,
            imdb_rating: row.get(15)?,
            video_type: row.get(16)?,
            season: row.get(17)?,
            episode: row.get(18)?,
            file_size: row.get(19)?,
            file_hash: row.get(20)?,
            created_at: row.get(21)?,
            updated_at: row.get(22)?,
        })
    }).map_err(|e| e.to_string())?;
    
    Ok(movie_result)
}

#[tauri::command]
pub async fn download_poster(app_handle: tauri::AppHandle, db: State<'_, Database>, movie_id: i64, poster_url: String) -> Result<String, String> {
    log::info!("Downloading poster for movie {} from {}", movie_id, poster_url);
    
    if poster_url.is_empty() {
        return Err("Poster URL is empty".to_string());
    }
    
    // Get cache directory
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let cache_dir = app_dir.join("cache").join("posters");
    fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    
    // Download image
    let client = reqwest::Client::new();
    let response = client.get(&poster_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;
    
    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    // Determine file extension
    let ext = if poster_url.contains(".png") {
        "png"
    } else {
        "jpg"
    };
    
    // Save to cache
    let filename = format!("{}_{}.{}", movie_id, uuid::Uuid::new_v4(), ext);
    let filepath = cache_dir.join(&filename);
    fs::write(&filepath, &bytes).map_err(|e| e.to_string())?;

    // Update database
    let filepath_str = filepath.to_string_lossy().to_string();

    // Verify file exists
    if !filepath.exists() {
        return Err(format!("Poster file was not created: {:?}", filepath));
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE movies SET poster_path = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![&filepath_str, movie_id],
    ).map_err(|e| e.to_string())?;

    log::info!("Poster saved successfully: {:?}, size: {} bytes", filepath, bytes.len());
    Ok(filepath_str)
}

#[tauri::command]
pub fn generate_nfo(db: State<'_, Database>, movie_id: i64) -> Result<String, String> {
    let movie = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, cnname, cnoname, year, countries, douban_id, imdb_id, 
             description, douban_rating, imdb_rating, video_type, season, episode
             FROM movies WHERE id = ?1"
        ).map_err(|e| e.to_string())?;
        
        stmt.query_row([movie_id], |row| {
            Ok(crate::models::Movie {
                id: row.get(0)?,
                directory_id: 0,
                series_id: None,
                filename: String::new(),
                path: String::new(),
                cnname: row.get(1)?,
                cnoname: row.get(2)?,
                year: row.get(4)?,
                countries: row.get(5)?,
                douban_id: row.get(6)?,
                imdb_id: row.get(7)?,
                poster_path: None,
                fanart_path: None,
                description: row.get(8)?,
                douban_rating: row.get(9)?,
                imdb_rating: row.get(10)?,
                video_type: row.get(11)?,
                season: row.get(12)?,
                episode: row.get(13)?,
                file_size: None,
                file_hash: None,
                created_at: String::new(),
                updated_at: String::new(),
            })
        }).map_err(|e| e.to_string())?
    };
    
    // Generate NFO content using new NFO module
    let nfo_content = services::nfo::generate_nfo_from_movie(&movie);
    
    // Get movie path to determine where to save NFO
    let (movie_path, _video_type) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT path, video_type FROM movies WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        stmt.query_row([movie_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|e| e.to_string())?
    };
    
    let path = Path::new(&movie_path);
    let directory = path.parent().ok_or("Invalid movie path")?;
    
    // Determine NFO filename
    let nfo_filename = if movie.video_type == "tv" {
        "tvshow.nfo"
    } else {
        "movie.nfo"
    };
    
    let nfo_path = directory.join(nfo_filename);
    services::nfo::save_nfo(&nfo_path, &nfo_content)?;
    
    log::info!("Generated NFO at: {:?}", nfo_path);
    Ok(nfo_path.to_string_lossy().to_string())
}

// TMDB 相关命令

#[derive(Debug, Serialize, Deserialize)]
pub struct TMDBSearchResult {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub year: Option<String>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub vote_average: f64,
    pub video_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMDBMovieDetail {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub cn_title: Option<String>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub year: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: f64,
    pub genres: Vec<String>,
    pub countries: Vec<String>,
    pub imdb_id: Option<String>,
}

/// 从数据库获取 TMDB API Key
fn get_tmdb_api_key_from_db(db: &Database) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = 'tmdb_api_key'")
        .map_err(|e| e.to_string())?;
    
    let api_key: Option<String> = stmt
        .query_row([], |row| row.get(0))
        .ok();
    
    // 如果没有设置，使用默认的测试 Key
    Ok(api_key.unwrap_or_else(|| {
        "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJiYjA1MmFkZmRkM2E4NjI4YzkyZWQyOTFkZjM0ZGUxNiIsIm5iZiI6MTczOTg0OTYyNy4yMjQsInN1YiI6IjY3YjVjMTE1NDAyMjQ5Yjc3NzY3ZWEwOCIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.F9V9h6GNq_sGjqj2nTCu5PhMrbV8j1yxe8V6UQqZyVk".to_string()
    }))
}

/// 使用 TMDB 搜索影片
#[tauri::command]
pub async fn search_tmdb(
    db: State<'_, Database>,
    title: String,
    year: Option<i32>,
    video_type: String,
) -> Result<Vec<TMDBSearchResult>, String> {
    log::info!("Searching TMDB for: {} ({:?}) type: {}", title, year, video_type);
    
    let api_key = get_tmdb_api_key_from_db(&db)?;
    let client = crate::services::tmdb::TMDBClient::new(api_key);
    
    let results: Vec<TMDBSearchResult> = if video_type == "tv" {
        client.search_tv(&title, year).await?
            .into_iter()
            .map(|r| TMDBSearchResult {
                id: r.id,
                title: r.name.clone(),
                original_title: r.original_name.clone(),
                year: r.first_air_date.as_ref().and_then(|d| d.split('-').next().map(|s| s.to_string())),
                overview: r.overview.clone(),
                poster_url: r.poster_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/w200{}", p)),
                vote_average: r.vote_average,
                video_type: "tv".to_string(),
            })
            .collect()
    } else {
        client.search_movie(&title, year).await?
            .into_iter()
            .map(|r| TMDBSearchResult {
                id: r.id,
                title: r.title.clone(),
                original_title: r.original_title.clone(),
                year: r.release_date.as_ref().and_then(|d| d.split('-').next().map(|s| s.to_string())),
                overview: r.overview.clone(),
                poster_url: r.poster_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/w200{}", p)),
                vote_average: r.vote_average,
                video_type: "movie".to_string(),
            })
            .collect()
    };
    
    log::info!("TMDB search found {} results", results.len());
    Ok(results)
}

/// 获取 TMDB 影片详情
#[tauri::command]
pub async fn get_tmdb_detail(
    db: State<'_, Database>,
    tmdb_id: i64,
    video_type: String,
) -> Result<TMDBMovieDetail, String> {
    log::info!("Getting TMDB detail for ID: {} type: {}", tmdb_id, video_type);
    
    let api_key = get_tmdb_api_key_from_db(&db)?;
    
    let client = crate::services::tmdb::TMDBClient::new(api_key);
    
    // 获取中文标题
    let cn_title = client.get_chinese_title(tmdb_id, video_type == "tv").await.ok().flatten();
    
    if video_type == "tv" {
        let detail = client.get_tv_detail(tmdb_id).await?;
        
        Ok(TMDBMovieDetail {
            id: detail.id,
            title: detail.name.clone(),
            original_title: detail.original_name.clone(),
            cn_title,
            overview: detail.overview.clone(),
            poster_url: detail.poster_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
            backdrop_url: detail.backdrop_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/original{}", p)),
            year: detail.first_air_date.as_ref().and_then(|d| d.split('-').next().map(|s| s.to_string())),
            runtime: detail.episode_run_time.first().copied(),
            vote_average: detail.vote_average,
            genres: detail.genres.iter().map(|g| g.name.clone()).collect(),
            countries: detail.origin_country.clone(),
            imdb_id: detail.external_ids.as_ref().and_then(|e| e.imdb_id.clone()),
        })
    } else {
        let detail = client.get_movie_detail(tmdb_id).await?;
        
        Ok(TMDBMovieDetail {
            id: detail.id,
            title: detail.title.clone(),
            original_title: detail.original_title.clone(),
            cn_title,
            overview: detail.overview.clone(),
            poster_url: detail.poster_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
            backdrop_url: detail.backdrop_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/original{}", p)),
            year: detail.release_date.as_ref().and_then(|d| d.split('-').next().map(|s| s.to_string())),
            runtime: detail.runtime,
            vote_average: detail.vote_average,
            genres: detail.genres.iter().map(|g| g.name.clone()).collect(),
            countries: detail.production_countries.iter().map(|c| c.name.clone()).collect(),
            imdb_id: detail.imdb_id.clone(),
        })
    }
}

/// 下载 TMDB 海报
#[tauri::command]
pub async fn download_tmdb_poster(
    app_handle: tauri::AppHandle,
    db: State<'_, Database>,
    movie_id: i64,
    poster_url: String,
) -> Result<String, String> {
    log::info!("Downloading TMDB poster for movie {} from: {}", movie_id, poster_url);
    
    // 获取 cache 目录
    let cache_dir = app_handle.path().app_cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;
    
    // 生成海报文件名
    let poster_filename = format!("poster_{}.jpg", movie_id);
    let poster_path = cache_dir.join(&poster_filename);
    
    // 下载海报
    let client = reqwest::Client::new();
    let response = client.get(&poster_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download poster: {}", e))?;
    
    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to get poster bytes: {}", e))?;
    
    std::fs::write(&poster_path, &bytes)
        .map_err(|e| format!("Failed to save poster: {}", e))?;

    // Verify file exists
    if !poster_path.exists() {
        return Err(format!("Poster file was not created: {:?}", poster_path));
    }

    let poster_path_str = poster_path.to_string_lossy().to_string();

    // 更新数据库
    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE movies SET poster_path = ?1 WHERE id = ?2",
            [
                &poster_path_str,
                &movie_id.to_string(),
            ],
        ).map_err(|e| e.to_string())?;
    }

    log::info!("TMDB Poster saved successfully: {:?}, size: {} bytes", poster_path, bytes.len());
    Ok(poster_path_str)
}

/// 使用 TMDB 信息更新影片
#[tauri::command]
pub async fn update_movie_from_tmdb(
    db: State<'_, Database>,
    movie_id: i64,
    tmdb_detail: TMDBMovieDetail,
) -> Result<(), String> {
    log::info!("Updating movie {} from TMDB data", movie_id);
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let cnname = tmdb_detail.cn_title.clone().unwrap_or(tmdb_detail.title.clone());
    let rating = tmdb_detail.vote_average.to_string();
    let year = tmdb_detail.year.clone().unwrap_or_default();
    let overview = tmdb_detail.overview.clone().unwrap_or_default();
    let imdb_id = tmdb_detail.imdb_id.clone().unwrap_or_default();
    
    conn.execute(
        "UPDATE movies SET 
         cnname = ?1, 
         cnoname = ?2, 
         year = ?3, 
         description = ?4, 
         douban_rating = ?5,
         imdb_id = ?6,
         updated_at = datetime('now')
         WHERE id = ?7",
        [
            &cnname,
            &tmdb_detail.original_title,
            &year,
            &overview,
            &rating,
            &imdb_id,
            &movie_id.to_string(),
        ],
    ).map_err(|e| e.to_string())?;
    
    log::info!("Movie {} updated successfully", movie_id);
    Ok(())
}

/// 使用系统默认播放器打开影片文件
#[tauri::command]
pub async fn open_movie_file(path: String) -> Result<(), String> {
    log::info!("Opening movie file: {}", path);
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    Ok(())
}

/// 删除影片记录
#[tauri::command]
pub fn delete_movie(db: State<Database>, movie_id: i64) -> Result<(), String> {
    log::info!("Deleting movie: {}", movie_id);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM movies WHERE id = ?1",
        [&movie_id],
    ).map_err(|e| e.to_string())?;

    log::info!("Movie {} deleted successfully", movie_id);
    Ok(())
}

/// 智能更新相关影片（同名不同季/集）
#[derive(Debug, Serialize)]
pub struct SmartUpdateResult {
    pub movie_id: i64,
    pub movie_name: String,
    pub season: Option<String>,
    pub episode: Option<String>,
}

#[tauri::command]
pub async fn smart_update_related_movies(
    app_handle: tauri::AppHandle,
    db: State<'_, Database>,
    source_movie_id: i64,
    tmdb_detail: TMDBMovieDetail,
) -> Result<Vec<SmartUpdateResult>, String> {
    log::info!("Starting smart update for related movies, source: {}", source_movie_id);

    // 获取源影片的 cnname, video_type 和 filename
    let (source_cnname, source_video_type, source_filename) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT cnname, video_type, filename FROM movies WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let (cnname, video_type, filename): (Option<String>, String, String) = stmt.query_row(
            [&source_movie_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        ).map_err(|e| format!("Source movie not found: {}", e))?;

        (cnname, video_type, filename)
    };

    // 提取基础名称用于文件名匹配
    let base_name = source_cnname.clone()
        .unwrap_or_else(|| extract_base_name(&source_filename));

    // 收集所有相关影片
    let mut related_movies: Vec<(i64, String, Option<String>, Option<String>)> = vec![];

    // 1. 基于 cnname 匹配（如果存在）
    if let Some(ref cn) = source_cnname {
        if !cn.is_empty() {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            let mut stmt = conn.prepare(
                "SELECT id, filename, season, episode FROM movies
                 WHERE cnname = ?1 AND video_type = ?2 AND id != ?3"
            ).map_err(|e| e.to_string())?;

            let results: Vec<_> = stmt.query_map(
                [cn, &source_video_type, &source_movie_id.to_string()],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                }
            )
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

            for movie in results {
                related_movies.push(movie);
            }
        }
    }

    // 2. 基于文件名相似度匹配
    if base_name.len() >= 3 {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, filename, season, episode FROM movies
             WHERE video_type = ?1 AND id != ?2 AND (cnname IS NULL OR cnname = '')"
        ).map_err(|e| e.to_string())?;

        let candidates: Vec<_> = stmt.query_map(
            [&source_video_type, &source_movie_id.to_string()],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            }
        )
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

        for (movie_id, filename, season, episode) in candidates {
            let candidate_base = extract_base_name(&filename);
            let score = similarity_score(&base_name, &candidate_base);

            // 相似度阈值 0.6
            if score >= 0.6 && !related_movies.iter().any(|(id, _, _, _)| *id == movie_id) {
                log::info!("Found similar movie by filename: {} (score: {:.2})", filename, score);
                related_movies.push((movie_id, filename, season, episode));
            }
        }
    }

    if related_movies.is_empty() {
        log::info!("No related movies found");
        return Ok(vec![]);
    }

    log::info!("Found {} related movies to update", related_movies.len());

    log::info!("Found {} related movies to update", related_movies.len());

    let mut updated_results = vec![];

    // 为每个相关影片更新信息
    for (movie_id, filename, season, episode) in related_movies {
        // 下载海报（如果有）
        let poster_path = if let Some(ref poster_url) = tmdb_detail.poster_url {
            let cache_dir = app_handle.path().app_cache_dir()
                .map_err(|e| format!("Failed to get cache dir: {}", e))?;
            let poster_filename = format!("poster_{}.jpg", movie_id);
            let poster_path = cache_dir.join(&poster_filename);

            // 异步下载海报
            let client = reqwest::Client::new();
            match client.get(poster_url).send().await {
                Ok(response) => {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if std::fs::write(&poster_path, bytes).is_ok() {
                                Some(poster_path.to_string_lossy().to_string())
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        } else {
            None
        };

        // 更新数据库
        {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            let cnname = tmdb_detail.cn_title.clone().unwrap_or(tmdb_detail.title.clone());
            let rating = tmdb_detail.vote_average.to_string();
            let year = tmdb_detail.year.clone().unwrap_or_default();
            let overview = tmdb_detail.overview.clone().unwrap_or_default();
            let imdb_id = tmdb_detail.imdb_id.clone().unwrap_or_default();

            conn.execute(
                "UPDATE movies SET
                 cnname = ?1,
                 cnoname = ?2,
                 year = ?3,
                 description = ?4,
                 douban_rating = ?5,
                 imdb_id = ?6,
                 poster_path = ?7,
                 updated_at = datetime('now')
                 WHERE id = ?8",
                [
                    &cnname,
                    &tmdb_detail.original_title,
                    &year,
                    &overview,
                    &rating,
                    &imdb_id,
                    &poster_path.unwrap_or_default(),
                    &movie_id.to_string(),
                ],
            ).map_err(|e| e.to_string())?;
        }

        updated_results.push(SmartUpdateResult {
            movie_id,
            movie_name: filename,
            season,
            episode,
        });

        log::info!("Updated related movie: {}", movie_id);
    }

    log::info!("Smart update completed, updated {} movies", updated_results.len());
    Ok(updated_results)
}

/// 从文件名中提取基础名称（去除季集信息、年份等）
fn extract_base_name(filename: &str) -> String {
    // 移除文件扩展名
    let name = filename.rsplitn(2, '.').nth(1).unwrap_or(filename);

    // 常见的季集模式
    let patterns = [
        regex::Regex::new(r"(?i)[.\s_-]*[Ss]\d{1,2}[Ee]\d{1,2}").unwrap(),           // S01E01
        regex::Regex::new(r"(?i)[.\s_-]*\d{1,2}[Xx]\d{1,2}").unwrap(),              // 1x01
        regex::Regex::new(r"(?i)[.\s_-]*第\d{1,2}[季集]").unwrap(),                   // 第1季, 第1集
        regex::Regex::new(r"(?i)[.\s_-]*\(\d{4}\)").unwrap(),                       // (2023)
        regex::Regex::new(r"(?i)[.\s_-]*\d{4}").unwrap(),                           // 2023
        regex::Regex::new(r"(?i)[.\s_-]*EP?\d{1,3}").unwrap(),                       // E01, EP01
        regex::Regex::new(r"(?i)[.\s_-]*\d{3,4}[Pp]").unwrap(),                      // 1080p, 720P
        regex::Regex::new(r"(?i)[.\s_-]*(bluray|bdrip|brrip|webrip|dvdrip|hdtv|web-dl)").unwrap(),
        regex::Regex::new(r"(?i)[.\s_-]*(aac|ac3|dts|truehd|atmos)").unwrap(),
        regex::Regex::new(r"(?i)[.\s_-]*(x264|x265|hevc|h264|h265)").unwrap(),
    ];

    let mut result = name.to_string();
    for pattern in &patterns {
        result = pattern.replace_all(&result, "").to_string();
    }

    // 清理多余的分隔符和空格
    result = result
        .replace('.', " ")
        .replace('_', " ")
        .replace('-', " ");

    // 去除首尾空格并归一化空格
    let parts: Vec<&str> = result.split_whitespace().collect();
    result = parts.join(" ");

    result.trim().to_lowercase()
}

/// 计算两个字符串的相似度（简单实现）
fn similarity_score(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    // 包含关系
    if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) {
        let len_ratio = a_lower.len().min(b_lower.len()) as f64 / a_lower.len().max(b_lower.len()) as f64;
        return 0.8 + len_ratio * 0.2;
    }

    // 简单的词频相似度
    let a_words: std::collections::HashSet<_> = a_lower.split_whitespace().collect();
    let b_words: std::collections::HashSet<_> = b_lower.split_whitespace().collect();

    let intersection: std::collections::HashSet<_> = a_words.intersection(&b_words).collect();
    let union: std::collections::HashSet<_> = a_words.union(&b_words).collect();

    if union.is_empty() {
        return 0.0;
    }

    intersection.len() as f64 / union.len() as f64
}

/// 基于文件名相似度的智能更新
#[tauri::command]
pub async fn smart_update_by_filename(
    app_handle: tauri::AppHandle,
    db: State<'_, Database>,
    source_movie_id: i64,
    tmdb_detail: TMDBMovieDetail,
) -> Result<Vec<SmartUpdateResult>, String> {
    log::info!("Starting filename-based smart update for movie: {}", source_movie_id);

    // 获取源影片信息
    let (source_filename, source_cnname, source_video_type, source_dir_id) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT filename, cnname, video_type, directory_id FROM movies WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let result: (String, Option<String>, String, i64) = stmt.query_row(
            [&source_movie_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(4)?))
        ).map_err(|e| format!("Source movie not found: {}", e))?;

        result
    };

    // 提取基础名称
    let base_name = source_cnname.clone()
        .unwrap_or_else(|| extract_base_name(&source_filename));

    if base_name.len() < 3 {
        log::info!("Base name too short, skipping filename-based update");
        return Ok(vec![]);
    }

    // 查找同一目录下的所有影片
    let candidates: Vec<(i64, String, Option<String>, Option<String>)> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, filename, season, episode FROM movies
             WHERE directory_id = ?1 AND id != ?2 AND video_type = ?3"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query(
            rusqlite::params![source_dir_id, source_movie_id, source_video_type]
        ).map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            results.push((
                row.get::<_, i64>(0).map_err(|e| e.to_string())?,
                row.get::<_, String>(1).map_err(|e| e.to_string())?,
                row.get::<_, Option<String>>(2).map_err(|e| e.to_string())?,
                row.get::<_, Option<String>>(3).map_err(|e| e.to_string())?,
            ));
        }
        results
    };

    // 基于相似度筛选
    let mut related_movies = vec![];
    for (movie_id, filename, season, episode) in candidates {
        let candidate_base = extract_base_name(&filename);
        let score = similarity_score(&base_name, &candidate_base);

        // 相似度阈值 0.6（可调整）
        if score >= 0.6 {
            log::info!("Found similar movie: {} (score: {:.2})", filename, score);
            related_movies.push((movie_id, filename, season, episode));
        }
    }

    // 如果有 cnname，也查找同 cnname 的影片
    if let Some(ref cn) = source_cnname {
        let same_cnname: Vec<(i64, String, Option<String>, Option<String>)> = {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            let mut stmt = conn.prepare(
                "SELECT id, filename, season, episode FROM movies
                 WHERE cnname = ?1 AND id != ?2"
            ).map_err(|e| e.to_string())?;

            let mut rows = stmt.query(
                rusqlite::params![cn, source_movie_id]
            ).map_err(|e| e.to_string())?;

            let mut results = Vec::new();
            while let Some(row) = rows.next().map_err(|e| e.to_string())? {
                results.push((
                    row.get::<_, i64>(0).map_err(|e| e.to_string())?,
                    row.get::<_, String>(1).map_err(|e| e.to_string())?,
                    row.get::<_, Option<String>>(2).map_err(|e| e.to_string())?,
                    row.get::<_, Option<String>>(3).map_err(|e| e.to_string())?,
                ));
            }
            results
        };

        // 合并结果，避免重复
        for movie in same_cnname {
            if !related_movies.iter().any(|(id, _, _, _)| *id == movie.0) {
                related_movies.push(movie);
            }
        }
    }

    if related_movies.is_empty() {
        log::info!("No related movies found by filename similarity");
        return Ok(vec![]);
    }

    log::info!("Found {} related movies by filename similarity", related_movies.len());

    // 更新相关影片（复用原有逻辑）
    let mut updated_results = vec![];

    for (movie_id, filename, season, episode) in related_movies {
        // 下载海报
        let poster_path = if let Some(ref poster_url) = tmdb_detail.poster_url {
            let cache_dir = app_handle.path().app_cache_dir()
                .map_err(|e| format!("Failed to get cache dir: {}", e))?;
            let poster_filename = format!("poster_{}.jpg", movie_id);
            let poster_path = cache_dir.join(&poster_filename);

            let client = reqwest::Client::new();
            match client.get(poster_url).send().await {
                Ok(response) => {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if std::fs::write(&poster_path, bytes).is_ok() {
                                Some(poster_path.to_string_lossy().to_string())
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        } else {
            None
        };

        // 更新数据库
        {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            let cnname = tmdb_detail.cn_title.clone().unwrap_or(tmdb_detail.title.clone());
            let rating = tmdb_detail.vote_average.to_string();
            let year = tmdb_detail.year.clone().unwrap_or_default();
            let overview = tmdb_detail.overview.clone().unwrap_or_default();
            let imdb_id = tmdb_detail.imdb_id.clone().unwrap_or_default();

            conn.execute(
                "UPDATE movies SET
                 cnname = ?1,
                 cnoname = ?2,
                 year = ?3,
                 description = ?4,
                 douban_rating = ?5,
                 imdb_id = ?6,
                 poster_path = ?7,
                 updated_at = datetime('now')
                 WHERE id = ?8",
                [
                    &cnname,
                    &tmdb_detail.original_title,
                    &year,
                    &overview,
                    &rating,
                    &imdb_id,
                    &poster_path.unwrap_or_default(),
                    &movie_id.to_string(),
                ],
            ).map_err(|e| e.to_string())?;
        }

        updated_results.push(SmartUpdateResult {
            movie_id,
            movie_name: filename,
            season,
            episode,
        });

        log::info!("Updated related movie: {}", movie_id);
    }

    log::info!("Filename-based smart update completed, updated {} movies", updated_results.len());
    Ok(updated_results)
}

#[tauri::command]
pub fn get_poster_image(poster_path: String) -> Result<Vec<u8>, String> {
    if poster_path.is_empty() {
        return Err("Poster path is empty".to_string());
    }

    let path = Path::new(&poster_path);
    if !path.exists() {
        return Err(format!("Poster file not found: {}", poster_path));
    }

    fs::read(path).map_err(|e| format!("Failed to read poster: {}", e))
}

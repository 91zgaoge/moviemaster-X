use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::collections::HashMap;

/// PT-Depiler search result
#[derive(Debug, Serialize, Clone)]
pub struct PTSearchResult {
    pub site: String,
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<String>,
    pub torrent_url: String,
    pub download_url: String,
    pub size: String,
    pub seeders: i32,
    pub leechers: i32,
    pub snatched: i32,
    pub freeleech: bool,
    pub double_upload: bool,
    pub publish_time: String,
    pub category: String,
    pub imdb_id: Option<String>,
    pub douban_id: Option<String>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
}

/// PT site configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PTSiteConfig {
    pub name: String,
    pub enabled: bool,
    pub api_url: String,
    pub passkey: String,
    pub cookie: String,
    pub auth_type: String, // "passkey", "cookie", "api_key"
}

/// Search request from PT-Depiler
#[derive(Debug, Deserialize)]
pub struct PTSearchRequest {
    pub keyword: String,
    pub year: Option<String>,
    pub video_type: Option<String>, // "movie" | "tv"
    pub season: Option<String>,
    pub episode: Option<String>,
    pub imdb_id: Option<String>,
}

/// HTTP bridge for PT-Depiler browser extension
/// This allows browser extension to search PT sites through MovieMaster
#[tauri::command]
pub async fn pt_depiler_search(
    db: State<'_, Database>,
    request: PTSearchRequest,
) -> Result<Vec<PTSearchResult>, String> {
    log::info!("PT-Depiler search: {:?}", request);

    // Load PT site configurations from database
    let sites = load_pt_sites(&db)?;

    let mut all_results = Vec::new();

    // Search enabled sites
    for site in sites {
        if !site.enabled {
            continue;
        }

        match search_pt_site(&site, &request).await {
            Ok(results) => {
                log::info!("Found {} results from {}", results.len(), site.name);
                all_results.extend(results);
            }
            Err(e) => {
                log::error!("Failed to search {}: {}", site.name, e);
            }
        }
    }

    // Sort by seeders (descending)
    all_results.sort_by(|a, b| b.seeders.cmp(&a.seeders));

    Ok(all_results)
}

/// Get PT site list (without credentials)
#[tauri::command]
pub fn pt_depiler_get_sites(db: State<Database>) -> Result<Vec<String>, String> {
    let sites = load_pt_sites(&db)?;
    Ok(sites.into_iter()
        .filter(|s| s.enabled)
        .map(|s| s.name)
        .collect())
}

/// Add PT site configuration
#[tauri::command]
pub fn pt_depiler_add_site(
    db: State<Database>,
    config: PTSiteConfig,
) -> Result<(), String> {
    log::info!("Adding PT site: {}", config.name);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Store config as JSON in settings
    let config_json = serde_json::to_string(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    let key = format!("pt_site_{}", config.name);

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        [&key, &config_json],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

/// Remove PT site
#[tauri::command]
pub fn pt_depiler_remove_site(
    db: State<Database>,
    site_name: String,
) -> Result<(), String> {
    log::info!("Removing PT site: {}", site_name);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let key = format!("pt_site_{}", site_name);

    conn.execute(
        "DELETE FROM settings WHERE key = ?1",
        [&key],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

/// Load all PT site configurations
fn load_pt_sites(db: &Database) -> Result<Vec<PTSiteConfig>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT value FROM settings WHERE key LIKE 'pt_site_%'"
    ).map_err(|e| e.to_string())?;

    let configs: Vec<PTSiteConfig> = stmt.query_map([], |row| {
        let json: String = row.get(0)?;
        let config: PTSiteConfig = serde_json::from_str(&json)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(config)
    })
    .map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    Ok(configs)
}

/// Search a specific PT site
async fn search_pt_site(
    site: &PTSiteConfig,
    request: &PTSearchRequest,
) -> Result<Vec<PTSearchResult>, String> {
    // This is a placeholder implementation
    // In production, you would implement the actual API for each PT site

    // Example implementation for a generic PT site
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("keyword", request.keyword.clone());
    if let Some(year) = &request.year {
        params.insert("year", year.clone());
    }
    if let Some(video_type) = &request.video_type {
        params.insert("type", video_type.clone());
    }

    // Add authentication
    match site.auth_type.as_str() {
        "passkey" => {
            params.insert("passkey", site.passkey.clone());
        }
        "api_key" => {
            // API key would go in header
        }
        "cookie" => {
            // Cookie would go in headers
        }
        _ => {}
    }

    // Make the request
    let response = client
        .get(&site.api_url)
        .query(&params)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    // Parse response (placeholder)
    // In production, parse the actual JSON/XML response from the PT site
    let results = parse_pt_response(response, site).await?;

    Ok(results)
}

/// Parse PT site response
async fn parse_pt_response(
    response: reqwest::Response,
    site: &PTSiteConfig,
) -> Result<Vec<PTSearchResult>, String> {
    // Placeholder implementation
    // Parse the response based on the site's API format

    let _text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // For now, return empty results
    // In production, implement actual parsing for each PT site's format
    log::warn!("PT site {} response parsing not implemented yet", site.name);

    Ok(Vec::new())
}

/// Test PT site connection
#[tauri::command]
pub async fn pt_depiler_test_site(
    site_name: String,
    api_url: String,
    auth_type: String,
    passkey: Option<String>,
    cookie: Option<String>,
) -> Result<bool, String> {
    log::info!("Testing PT site connection: {}", site_name);

    let client = reqwest::Client::new();

    let mut request = client.get(&api_url);

    // Add authentication headers
    match auth_type.as_str() {
        "cookie" => {
            if let Some(cookie) = cookie {
                request = request.header("Cookie", cookie);
            }
        }
        "api_key" => {
            if let Some(key) = passkey {
                request = request.header("Authorization", format!("Bearer {}", key));
            }
        }
        _ => {}
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    Ok(response.status().is_success())
}

/// Download torrent file from PT site
#[tauri::command]
pub async fn pt_depiler_download_torrent(
    site_name: String,
    torrent_url: String,
    save_path: String,
) -> Result<String, String> {
    log::info!("Downloading torrent from {}: {}", site_name, torrent_url);

    let client = reqwest::Client::new();

    let response = client
        .get(&torrent_url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let bytes = response.bytes().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Save to file
    std::fs::write(&save_path, &bytes)
        .map_err(|e| format!("Failed to save file: {}", e))?;

    log::info!("Torrent saved to: {}", save_path);
    Ok(save_path)
}

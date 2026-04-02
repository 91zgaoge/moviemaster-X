use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// qBittorrent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QBConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub default_save_path: Option<String>,
}

impl Default for QBConfig {
    fn default() -> Self {
        Self {
            base_url: "http://10.40.31.69:8044".to_string(),
            username: String::new(),
            password: String::new(),
            default_save_path: None,
        }
    }
}

/// qBittorrent client
pub struct QBClient {
    client: reqwest::Client,
    config: QBConfig,
    cookie: Arc<RwLock<Option<String>>>,
}

impl QBClient {
    pub fn new(config: QBConfig) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            config,
            cookie: Arc::new(RwLock::new(None)),
        }
    }

    /// Login to qBittorrent
    pub async fn login(&self) -> Result<(), String> {
        let login_url = format!("{}/api/v2/auth/login", self.config.base_url);

        let params = [
            ("username", self.config.username.as_str()),
            ("password", self.config.password.as_str()),
        ];

        let response = self.client
            .post(&login_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;

        if response.status().is_success() {
            // Extract cookie from response
            if let Some(cookie) = response.headers().get("set-cookie") {
                let cookie_str = cookie.to_str()
                    .map_err(|e| format!("Invalid cookie: {}", e))?;
                let mut cookie_guard = self.cookie.write().await;
                *cookie_guard = Some(cookie_str.split(';').next().unwrap_or("").to_string());
                log::info!("qBittorrent login successful");
                Ok(())
            } else {
                Err("No cookie in response".to_string())
            }
        } else {
            Err(format!("Login failed: {}", response.status()))
        }
    }

    /// Add torrent from URL
    pub async fn add_torrent_url(&self, url: &str, save_path: Option<&str>) -> Result<(), String> {
        self.ensure_logged_in().await?;

        let add_url = format!("{}/api/v2/torrents/add", self.config.base_url);

        let mut form = vec![
            ("urls", url),
        ];

        if let Some(path) = save_path {
            form.push(("savepath", path));
        }

        let cookie = self.cookie.read().await.clone()
            .ok_or("Not logged in")?;

        let response = self.client
            .post(&add_url)
            .header("Cookie", cookie)
            .form(&form)
            .send()
            .await
            .map_err(|e| format!("Add torrent failed: {}", e))?;

        if response.status().is_success() {
            log::info!("Torrent added successfully: {}", url);
            Ok(())
        } else {
            Err(format!("Add torrent failed: {}", response.status()))
        }
    }

    /// Add torrent from file
    pub async fn add_torrent_file(&self, file_path: &str, save_path: Option<&str>) -> Result<(), String> {
        self.ensure_logged_in().await?;

        let add_url = format!("{}/api/v2/torrents/add", self.config.base_url);

        // Read file
        let file_content = std::fs::read(file_path)
            .map_err(|e| format!("Failed to read torrent file: {}", e))?;

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("torrent.torrent");

        let form = reqwest::multipart::Form::new()
            .part("torrents", reqwest::multipart::Part::bytes(file_content)
                .file_name(file_name.to_string()));

        let form = if let Some(path) = save_path {
            form.text("savepath", path.to_string())
        } else {
            form
        };

        let cookie = self.cookie.read().await.clone()
            .ok_or("Not logged in")?;

        let response = self.client
            .post(&add_url)
            .header("Cookie", cookie)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Add torrent file failed: {}", e))?;

        if response.status().is_success() {
            log::info!("Torrent file added successfully: {}", file_path);
            Ok(())
        } else {
            Err(format!("Add torrent file failed: {}", response.status()))
        }
    }

    /// Get torrent list
    pub async fn get_torrents(&self) -> Result<Vec<QBTorrent>, String> {
        self.ensure_logged_in().await?;

        let list_url = format!("{}/api/v2/torrents/info", self.config.base_url);

        let cookie = self.cookie.read().await.clone()
            .ok_or("Not logged in")?;

        let response = self.client
            .get(&list_url)
            .header("Cookie", cookie)
            .send()
            .await
            .map_err(|e| format!("Get torrents failed: {}", e))?;

        if response.status().is_success() {
            let torrents: Vec<QBTorrent> = response.json().await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(torrents)
        } else {
            Err(format!("Get torrents failed: {}", response.status()))
        }
    }

    /// Ensure logged in (auto re-login if needed)
    async fn ensure_logged_in(&self) -> Result<(), String> {
        let has_cookie = self.cookie.read().await.is_some();
        if !has_cookie {
            self.login().await?;
        }
        Ok(())
    }
}

/// qBittorrent torrent info
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QBTorrent {
    pub hash: String,
    pub name: String,
    pub size: i64,
    pub progress: f64,
    pub dlspeed: i64,
    pub upspeed: i64,
    pub state: String,
    pub category: String,
    pub save_path: String,
    pub added_on: i64,
    pub completion_on: i64,
}

/// Tauri commands
use tauri::State;
use crate::db::Database;

/// Test qBittorrent connection
#[tauri::command]
pub async fn qbittorrent_test_connection(
    base_url: String,
    username: String,
    password: String,
) -> Result<bool, String> {
    let config = QBConfig {
        base_url,
        username,
        password,
        default_save_path: None,
    };

    let client = QBClient::new(config);
    match client.login().await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Add torrent URL to qBittorrent
#[tauri::command]
pub async fn qbittorrent_add_torrent(
    db: State<'_, Database>,
    torrent_url: String,
    save_path: Option<String>,
) -> Result<(), String> {
    let config = load_qb_config(&db)?;
    let client = QBClient::new(config);

    client.add_torrent_url(&torrent_url, save_path.as_deref()).await
}

/// Get qBittorrent torrent list
#[tauri::command]
pub async fn qbittorrent_get_torrents(
    db: State<'_, Database>,
) -> Result<Vec<QBTorrent>, String> {
    let config = load_qb_config(&db)?;
    let client = QBClient::new(config);

    client.get_torrents().await
}

/// Save qBittorrent configuration
#[tauri::command]
pub fn qbittorrent_save_config(
    db: State<Database>,
    config: QBConfig,
) -> Result<(), String> {
    log::info!("Saving qBittorrent config: {}", config.base_url);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let config_json = serde_json::to_string(&config)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('qbittorrent_config', ?1)",
        [config_json],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

/// Load qBittorrent configuration
#[tauri::command]
pub fn qbittorrent_load_config(
    db: State<Database>,
) -> Result<QBConfig, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let result: Result<String, _> = conn.query_row(
        "SELECT value FROM settings WHERE key = 'qbittorrent_config'",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(json) => {
            let config: QBConfig = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse: {}", e))?;
            Ok(config)
        }
        Err(_) => Ok(QBConfig::default()),
    }
}

/// Helper function to load config
fn load_qb_config(db: &Database) -> Result<QBConfig, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let result: Result<String, _> = conn.query_row(
        "SELECT value FROM settings WHERE key = 'qbittorrent_config'",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(json) => {
            let config: QBConfig = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse: {}", e))?;
            Ok(config)
        }
        Err(_) => Err("qBittorrent not configured".to_string()),
    }
}
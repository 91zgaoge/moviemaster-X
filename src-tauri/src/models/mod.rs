use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub id: i64,
    pub path: String,
    pub name: Option<String>,
    pub path_type: String,
    pub smb_connection_id: Option<String>,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub directory_id: i64,
    pub series_id: Option<i64>,
    pub filename: String,
    pub path: String,
    pub cnname: Option<String>,
    pub cnoname: Option<String>,
    pub year: Option<String>,
    pub countries: Option<String>,
    pub douban_id: Option<String>,
    pub imdb_id: Option<String>,
    pub poster_path: Option<String>,
    pub fanart_path: Option<String>,
    pub description: Option<String>,
    pub douban_rating: Option<f64>,
    pub imdb_rating: Option<f64>,
    pub video_type: String,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBConnection {
    pub id: String,
    pub name: String,
    pub host: String,
    pub share: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub domain: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub id: i64,
    pub movie_id: i64,
    pub language: Option<String>,
    pub format: Option<String>,
    pub filename: Option<String>,
    pub path: Option<String>,
    pub download_url: Option<String>,
    pub file_hash: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: Option<String>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDirectoryRequest {
    pub path: String,
    pub name: Option<String>,
    pub path_type: Option<String>,
    pub smb_connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub total: usize,
    pub current: usize,
    pub current_file: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFilename {
    pub title: String,
    pub year: Option<String>,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub resolution: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub release_group: Option<String>,
    pub video_type: String, // "movie" or "tv"
}

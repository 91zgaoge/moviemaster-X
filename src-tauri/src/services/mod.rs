use crate::models::ParsedFilename;
use crate::scanner::FilenameParser;

pub mod tmdb;
pub mod nfo;
pub mod smb_client;
pub mod subtitle;
pub mod import_export;
pub mod scan_manager;
pub mod vnfo;

pub mod douban {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct DoubanSearchResult {
        pub subjects: Option<Vec<DoubanSubject>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct DoubanSubject {
        pub id: String,
        pub title: String,
        pub year: String,
        pub rating: Option<DoubanRating>,
        pub images: Option<DoubanImages>,
        pub genres: Option<Vec<String>>,
        pub directors: Option<Vec<DoubanPerson>>,
        pub casts: Option<Vec<DoubanPerson>>,
        pub aka: Option<Vec<String>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct DoubanRating {
        pub average: f64,
    }

    #[derive(Debug, Deserialize)]
    pub struct DoubanImages {
        pub small: String,
        pub large: String,
        pub medium: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct DoubanPerson {
        pub name: String,
        pub id: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DoubanSubjectDetail {
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
        pub durations: Vec<String>,
        pub pubdate: Option<String>,
        pub poster: Option<String>,
    }

    pub async fn search_movie(title: &str, year: Option<&str>) -> Result<Option<DoubanSubject>, String> {
        // Note: This is a placeholder. In production, you'd use a proper API
        // or implement web scraping with proper rate limiting
        log::info!("Searching Douban for: {} ({:?})", title, year);
        Ok(None)
    }

    pub async fn get_subject_detail(id: &str) -> Result<Option<DoubanSubjectDetail>, String> {
        log::info!("Getting Douban subject detail for ID: {}", id);
        Ok(None)
    }
}

pub mod hash {
    use sha2::{Digest, Sha256};
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    /// Calculate OpenSubtitles hash (first and last 64KB)
    pub fn calculate_opensubtitles_hash(path: &Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let file_size = file.metadata().map_err(|e| e.to_string())?.len();
        let mut reader = BufReader::new(file);
        
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 65536]; // 64KB
        
        // Read first 64KB
        let bytes_read = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        hasher.update(&buffer[..bytes_read]);
        
        // Read last 64KB if file is large enough
        if file_size > 65536 {
            let file = File::open(path).map_err(|e| e.to_string())?;
            let mut reader = BufReader::new(file);
            
            // Seek to offset
            std::io::Seek::seek(&mut reader, std::io::SeekFrom::End(-65536))
                .map_err(|e| e.to_string())?;
            
            let bytes_read = reader.read(&mut buffer).map_err(|e| e.to_string())?;
            hasher.update(&buffer[..bytes_read]);
        }
        
        // Include file size
        hasher.update(&file_size.to_le_bytes());
        
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}

pub fn parse_filename(filename: &str) -> ParsedFilename {
    FilenameParser::parse(filename)
}

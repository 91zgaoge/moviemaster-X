use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Directories table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS directories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                name TEXT,
                path_type TEXT DEFAULT 'local',
                smb_connection_id TEXT,
                enabled INTEGER DEFAULT 1,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Movies table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS movies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                directory_id INTEGER REFERENCES directories(id),
                filename TEXT NOT NULL,
                path TEXT NOT NULL,
                cnname TEXT,
                cnoname TEXT,
                year TEXT,
                countries TEXT,
                douban_id TEXT,
                imdb_id TEXT,
                poster_path TEXT,
                fanart_path TEXT,
                description TEXT,
                douban_rating REAL,
                imdb_rating REAL,
                video_type TEXT DEFAULT 'movie',
                season TEXT,
                episode TEXT,
                file_size INTEGER,
                file_hash TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // SMB connections table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS smb_connections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                share TEXT NOT NULL,
                username TEXT,
                password TEXT,
                domain TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Subtitles table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subtitles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                movie_id INTEGER REFERENCES movies(id),
                language TEXT,
                format TEXT,
                filename TEXT,
                path TEXT,
                download_url TEXT,
                file_hash TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
            [],
        )?;

        // Indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_movies_directory ON movies(directory_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_movies_cnname ON movies(cnname)",
            [],
        )?;

        Ok(())
    }
}

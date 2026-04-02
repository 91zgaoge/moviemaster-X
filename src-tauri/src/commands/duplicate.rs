use rusqlite::params;
use serde::{Deserialize, Serialize};
use crate::db::Database;
use tauri::State;

#[derive(Debug, Serialize, Clone)]
pub struct DuplicateGroup {
    pub key: String,
    pub match_type: String,
    pub movies: Vec<DuplicateMovie>,
    pub suggested_keep: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DuplicateMovie {
    pub id: i64,
    pub filename: String,
    pub path: String,
    pub cnname: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    pub video_type: String,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub poster_path: Option<String>,
    pub imdb_id: Option<String>,
    pub douban_rating: Option<f64>,
    pub created_at: String,
    pub completeness_score: i32,
}

#[tauri::command]
pub fn scan_duplicates(db: State<Database>) -> Result<Vec<DuplicateGroup>, String> {
    let mut groups = Vec::new();

    // Find duplicates by file_hash
    let hash_groups = find_by_hash(&db)?;
    groups.extend(hash_groups);

    Ok(groups)
}

fn find_by_hash(db: &Database) -> Result<Vec<DuplicateGroup>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT file_hash FROM movies
         WHERE file_hash IS NOT NULL AND file_hash != ''
         GROUP BY file_hash HAVING COUNT(*) > 1"
    ).map_err(|e| e.to_string())?;

    let hashes: Vec<String> = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })
    .map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    drop(stmt);

    let mut result = Vec::new();

    for hash in hashes {
        let mut stmt = conn.prepare(
            "SELECT id, filename, path, cnname, file_size, file_hash,
             video_type, season, episode, poster_path, imdb_id, douban_rating, created_at
             FROM movies WHERE file_hash = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query([&hash]).map_err(|e| e.to_string())?;
        let mut movies = Vec::new();

        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let id: i64 = row.get(0).map_err(|e| e.to_string())?;
            let filename: String = row.get(1).map_err(|e| e.to_string())?;
            let path: String = row.get(2).map_err(|e| e.to_string())?;
            let cnname: Option<String> = row.get(3).map_err(|e| e.to_string())?;
            let file_size: Option<i64> = row.get(4).map_err(|e| e.to_string())?;
            let file_hash: Option<String> = row.get(5).map_err(|e| e.to_string())?;
            let video_type: String = row.get(6).map_err(|e| e.to_string())?;
            let season: Option<String> = row.get(7).map_err(|e| e.to_string())?;
            let episode: Option<String> = row.get(8).map_err(|e| e.to_string())?;
            let poster_path: Option<String> = row.get(9).map_err(|e| e.to_string())?;
            let imdb_id: Option<String> = row.get(10).map_err(|e| e.to_string())?;
            let douban_rating: Option<f64> = row.get(11).map_err(|e| e.to_string())?;
            let created_at: String = row.get(12).map_err(|e| e.to_string())?;

            let score = calc_score(&cnname, poster_path.is_some(), imdb_id.is_some(), douban_rating, file_size);

            movies.push(DuplicateMovie {
                id, filename, path, cnname, file_size, file_hash,
                video_type, season, episode, poster_path, imdb_id,
                douban_rating, created_at, completeness_score: score,
            });
        }

        if movies.len() > 1 {
            let suggested = movies.iter()
                .max_by_key(|m| m.completeness_score)
                .map(|m| m.id)
                .unwrap_or(movies[0].id);

            result.push(DuplicateGroup {
                key: hash.clone(),
                match_type: "hash".to_string(),
                movies,
                suggested_keep: suggested,
            });
        }
    }

    Ok(result)
}

fn calc_score(
    cnname: &Option<String>,
    has_poster: bool,
    has_imdb: bool,
    rating: Option<f64>,
    size: Option<i64>,
) -> i32 {
    let mut score = 0;
    if cnname.as_ref().map_or(false, |s| !s.is_empty()) { score += 20; }
    if has_poster { score += 25; }
    if has_imdb { score += 20; }
    if rating.is_some() { score += 15; }
    if let Some(s) = size {
        let gb = s as f64 / 1024.0 / 1024.0 / 1024.0;
        if gb >= 10.0 { score += 20; }
        else if gb >= 5.0 { score += 15; }
        else if gb >= 2.0 { score += 10; }
        else if gb >= 1.0 { score += 5; }
    }
    score
}

#[tauri::command]
pub fn delete_duplicates(
    db: State<Database>,
    keep_id: i64,
    delete_ids: Vec<i64>,
) -> Result<usize, String> {
    let mut conn = db.conn.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let mut count = 0;
    for id in delete_ids {
        if id == keep_id { continue; }
        tx.execute("DELETE FROM movies WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        count += 1;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(count)
}

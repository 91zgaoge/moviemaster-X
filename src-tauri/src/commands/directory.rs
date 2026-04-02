use crate::db::Database;
use crate::models::{AddDirectoryRequest, Directory};
use tauri::State;

#[tauri::command]
pub fn get_directories(db: State<Database>) -> Result<Vec<Directory>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, path, name, path_type, smb_connection_id, enabled, created_at FROM directories ORDER BY id")
        .map_err(|e| e.to_string())?;
    
    let directories = stmt
        .query_map([], |row| {
            Ok(Directory {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                path_type: row.get::<_, String>(3).unwrap_or_else(|_| "local".to_string()),
                smb_connection_id: row.get(4)?,
                enabled: row.get::<_, i32>(5).unwrap_or(1) == 1,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    
    Ok(directories)
}

#[tauri::command]
pub fn add_directory(db: State<Database>, request: AddDirectoryRequest) -> Result<Directory, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let path_type = request.path_type.unwrap_or_else(|| "local".to_string());
    
    conn.execute(
        "INSERT INTO directories (path, name, path_type, smb_connection_id) VALUES (?1, ?2, ?3, ?4)",
        (&request.path, &request.name, &path_type, &request.smb_connection_id),
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    Ok(Directory {
        id,
        path: request.path,
        name: request.name,
        path_type,
        smb_connection_id: request.smb_connection_id,
        enabled: true,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub fn remove_directory(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // First delete associated movies
    conn.execute("DELETE FROM movies WHERE directory_id = ?1", [id])
        .map_err(|e| e.to_string())?;
    
    // Then delete the directory
    conn.execute("DELETE FROM directories WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn toggle_directory(db: State<Database>, id: i64, enabled: bool) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE directories SET enabled = ?1 WHERE id = ?2",
        (enabled as i32, id),
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

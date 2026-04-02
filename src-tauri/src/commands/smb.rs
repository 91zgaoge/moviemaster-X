use crate::db::Database;
use crate::models::SMBConnection;
use tauri::State;

#[tauri::command]
pub fn get_smb_connections(db: State<Database>) -> Result<Vec<SMBConnection>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, host, share, username, password, domain, created_at FROM smb_connections ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let connections = stmt
        .query_map([], |row| {
            Ok(SMBConnection {
                id: row.get(0)?,
                name: row.get(1)?,
                host: row.get(2)?,
                share: row.get(3)?,
                username: row.get(4)?,
                password: row.get(5)?,
                domain: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    
    Ok(connections)
}

#[tauri::command]
pub fn add_smb_connection(
    db: State<Database>,
    name: String,
    host: String,
    share: String,
    username: Option<String>,
    password: Option<String>,
    domain: Option<String>,
) -> Result<SMBConnection, String> {
    let id = uuid::Uuid::new_v4().to_string();
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO smb_connections (id, name, host, share, username, password, domain) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&id, &name, &host, &share, &username, &password, &domain),
    ).map_err(|e| e.to_string())?;
    
    Ok(SMBConnection {
        id,
        name,
        host,
        share,
        username,
        password,
        domain,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub fn remove_smb_connection(db: State<Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Delete associated directories first
    conn.execute("DELETE FROM directories WHERE smb_connection_id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    
    // Then delete the connection
    conn.execute("DELETE FROM smb_connections WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn test_smb_connection(
    host: String,
    share: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<bool, String> {
    // TODO: Implement actual SMB connection test
    // For now, just validate the inputs
    if host.is_empty() {
        return Err("Host cannot be empty".to_string());
    }
    if share.is_empty() {
        return Err("Share cannot be empty".to_string());
    }
    
    // Simulate connection test
    log::info!("Testing SMB connection to {} with share {}", host, share);
    Ok(true)
}

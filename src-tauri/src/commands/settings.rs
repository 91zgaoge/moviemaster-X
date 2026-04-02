use crate::db::Database;
use crate::models::Settings;
use tauri::State;

#[tauri::command]
pub fn get_settings(db: State<Database>) -> Result<Vec<Settings>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;
    
    let settings = stmt
        .query_map([], |row| {
            Ok(Settings {
                key: row.get(0)?,
                value: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    
    Ok(settings)
}

#[tauri::command]
pub fn update_setting(db: State<Database>, key: String, value: Option<String>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        (&key, &value),
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

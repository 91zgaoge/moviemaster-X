use crate::db::Database;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub version: String,
    pub export_time: String,
    pub settings: Vec<SettingItem>,
    pub directories: Vec<DirectoryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingItem {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryItem {
    pub path: String,
    pub name: Option<String>,
    pub path_type: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub settings_count: usize,
    pub directories_count: usize,
}

/// 导出配置到 JSON 文件
#[tauri::command]
pub async fn export_config(app_handle: tauri::AppHandle, db: State<'_, Database>) -> Result<String, String> {
    log::info!("Starting config export");

    // 先读取所有数据（在 await 之前）
    let (settings, directories) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;

        // 读取所有设置
        let mut settings_stmt = conn
            .prepare("SELECT key, value FROM settings")
            .map_err(|e| e.to_string())?;

        let settings: Vec<SettingItem> = settings_stmt
            .query_map([], |row| {
                Ok(SettingItem {
                    key: row.get(0)?,
                    value: row.get(1)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 读取所有目录
        let mut dir_stmt = conn
            .prepare("SELECT path, name, path_type, enabled FROM directories")
            .map_err(|e| e.to_string())?;

        let directories: Vec<DirectoryItem> = dir_stmt
            .query_map([], |row| {
                Ok(DirectoryItem {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    path_type: row.get(2)?,
                    enabled: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (settings, directories)
    }; // conn 在这里 drop

    // 构建配置数据
    let config = ConfigData {
        version: "1.0".to_string(),
        export_time: chrono::Local::now().to_rfc3339(),
        settings,
        directories,
    };

    // 序列化为 JSON
    let json_content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;

    // 使用文件对话框选择保存位置（使用 oneshot 将回调转为 async）
    let (tx, rx) = oneshot::channel::<Option<String>>();

    app_handle
        .dialog()
        .file()
        .set_file_name("moviemaster_config.json")
        .add_filter("JSON Files", &["json"])
        .save_file(move |file_path| {
            let path_str = file_path.map(|p| p.to_string());
            let _ = tx.send(path_str);
        });

    let path_str = rx.await
        .map_err(|e| format!("Dialog channel error: {}", e))?
        .ok_or("User cancelled")?;

    // 写入文件
    fs::write(&path_str, json_content).map_err(|e| format!("Failed to write file: {}", e))?;

    log::info!("Config exported to: {}", path_str);
    Ok(path_str)
}

/// 从 JSON 文件导入配置
#[tauri::command]
pub async fn import_config(app_handle: tauri::AppHandle, db: State<'_, Database>) -> Result<ImportResult, String> {
    log::info!("Starting config import");

    // 使用文件对话框选择文件（使用 oneshot 将回调转为 async）
    let (tx, rx) = oneshot::channel::<Option<String>>();

    app_handle
        .dialog()
        .file()
        .add_filter("JSON Files", &["json"])
        .pick_file(move |file_path| {
            let path_str = file_path.map(|p| p.to_string());
            let _ = tx.send(path_str);
        });

    let path_str = rx.await
        .map_err(|e| format!("Dialog channel error: {}", e))?
        .ok_or("User cancelled")?;

    // 读取文件内容
    let json_content = fs::read_to_string(&path_str).map_err(|e| format!("Failed to read file: {}", e))?;

    // 解析 JSON
    let config: ConfigData = serde_json::from_str(&json_content).map_err(|e| format!("Invalid JSON format: {}", e))?;

    // 验证版本
    if config.version != "1.0" {
        return Err(format!("Unsupported config version: {}", config.version));
    }

    // 导入到数据库
    let mut conn = db.conn.lock().map_err(|e| e.to_string())?;

    // 导入设置（直接替换）
    let settings_count = config.settings.len();
    for setting in &config.settings {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            (&setting.key, &setting.value),
        ).map_err(|e| e.to_string())?;
    }

    // 导入目录（跳过已存在的路径）
    let mut directories_count = 0;
    for dir in &config.directories {
        // 检查路径是否已存在
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM directories WHERE path = ?1",
                [&dir.path],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !exists {
            conn.execute(
                "INSERT INTO directories (path, name, path_type, enabled, created_at) VALUES (?1, ?2, ?3, ?4, datetime('now'))",
                (&dir.path, &dir.name, &dir.path_type, &dir.enabled),
            ).map_err(|e| e.to_string())?;
            directories_count += 1;
        }
    }

    log::info!(
        "Config imported: {} settings, {} directories added",
        settings_count,
        directories_count
    );

    Ok(ImportResult {
        settings_count,
        directories_count,
    })
}

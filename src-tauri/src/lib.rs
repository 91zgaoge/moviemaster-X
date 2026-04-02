use tauri::Manager;

mod agent;
mod commands;
mod db;
mod models;
mod scanner;
mod services;

pub use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting MovieMaster application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            log::info!("Initializing database...");
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
            
            // Create cache directory for images
            let cache_dir = app_dir.join("cache");
            std::fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
            
            let db_path = app_dir.join("moviemaster.db");
            log::info!("Database path: {:?}", db_path);
            
            let database = Database::new(&db_path).expect("Failed to initialize database");
            app.manage(database);

            // Initialize AI Agent state
            log::info!("Initializing AI Agent...");
            let agent_state = commands::agent::AgentState::new();
            app.manage(agent_state);

            // Show main window (created by tauri.conf.json)
            if let Some(window) = app.get_webview_window("main") {
                log::info!("Showing main window...");
                let _ = window.show();
            } else {
                log::warn!("Main window not found");
            }
            
            log::info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Directory commands
            commands::directory::get_directories,
            commands::directory::add_directory,
            commands::directory::remove_directory,
            commands::directory::toggle_directory,
            // Movie commands
            commands::movie::get_movies,
            commands::movie::scan_directory,
            commands::movie::get_movie_by_id,
            commands::movie::update_movie_info,
            commands::movie::search_douban,
            commands::movie::fetch_douban_info,
            commands::movie::download_poster,
            commands::movie::generate_nfo,
            // TMDB commands
            commands::movie::search_tmdb,
            commands::movie::get_tmdb_detail,
            commands::movie::download_tmdb_poster,
            commands::movie::update_movie_from_tmdb,
            commands::movie::smart_update_related_movies,
            commands::movie::smart_update_by_filename,
            commands::movie::open_movie_file,
            commands::movie::delete_movie,
            // Duplicate commands
            commands::duplicate::scan_duplicates,
            commands::duplicate::delete_duplicates,
            // PT-Depiler commands
            commands::pt_depiler::pt_depiler_search,
            commands::pt_depiler::pt_depiler_get_sites,
            commands::pt_depiler::pt_depiler_add_site,
            commands::pt_depiler::pt_depiler_remove_site,
            commands::pt_depiler::pt_depiler_test_site,
            commands::pt_depiler::pt_depiler_download_torrent,
            // Import/Export commands
            commands::import_export::export_movies_json,
            commands::import_export::export_movies_csv,
            commands::import_export::import_movies_json,
            commands::import_export::import_movies_csv,
            // Subtitle commands
            commands::subtitle::search_subtitles,
            commands::subtitle::download_subtitle,
            // SMB commands
            commands::smb::get_smb_connections,
            commands::smb::add_smb_connection,
            commands::smb::remove_smb_connection,
            commands::smb::test_smb_connection,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::update_setting,
            // Config export/import commands
            commands::config::export_config,
            commands::config::import_config,
            // qBittorrent commands
            commands::qbittorrent::qbittorrent_test_connection,
            commands::qbittorrent::qbittorrent_add_torrent,
            commands::qbittorrent::qbittorrent_get_torrents,
            commands::qbittorrent::qbittorrent_save_config,
            commands::qbittorrent::qbittorrent_load_config,
            // AI Agent commands
            commands::agent::agent_send_message,
            commands::agent::agent_get_metrics,
            commands::agent::agent_export_knowledge,
            commands::agent::agent_import_knowledge,
            commands::agent::agent_get_available_skills,
            commands::agent::agent_test_llm_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
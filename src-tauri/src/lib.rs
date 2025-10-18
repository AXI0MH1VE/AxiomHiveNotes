mod commands;
mod db;
mod models;

use db::{get_db_path, Database};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Initialize database
      let db_path = get_db_path(app.handle());
      let db = Database::new(db_path).expect("Failed to initialize database");
      db.initialize().expect("Failed to run migrations");
      
      app.manage(db);
      
      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_notification::init())
    .invoke_handler(tauri::generate_handler![
      commands::create_note,
      commands::get_note,
      commands::update_note,
      commands::delete_note,
      commands::list_notes,
      commands::search_notes,
      commands::create_tag,
      commands::list_tags,
      commands::assign_tags,
      commands::list_folders,
      commands::get_setting,
      commands::set_setting,
      commands::get_backlinks,
      commands::get_graph_data,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

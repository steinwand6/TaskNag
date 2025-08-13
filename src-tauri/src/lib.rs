pub mod commands;
pub mod database;
pub mod error;
pub mod models;
pub mod services;

use database::Database;
use services::TaskService;
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
      let handle = app.handle().clone();
      tauri::async_runtime::block_on(async move {
        let db = Database::new(&handle)
          .await
          .expect("Failed to initialize database");
        
        // Initialize services
        let task_service = TaskService::new(db);
        
        // Add services to app state
        app.manage(task_service);
      });
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::create_task,
      commands::get_tasks,
      commands::get_task_by_id,
      commands::update_task,
      commands::delete_task,
      commands::get_tasks_by_status,
      commands::move_task,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

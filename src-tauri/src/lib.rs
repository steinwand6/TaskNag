pub mod commands;
pub mod database;
pub mod error;
pub mod models;
pub mod services;

use database::Database;
use services::TaskService;
use tauri::{
  AppHandle, Manager, WindowEvent, 
  tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
  menu::{Menu, MenuItem, MenuEvent}
};

fn handle_tray_event(app: &AppHandle, event: TrayIconEvent) {
  match event {
    TrayIconEvent::Click { button, .. } => {
      match button {
        MouseButton::Left => {
          if let Some(window) = app.get_webview_window("main") {
            // シングルクリックでは表示のみ（非表示にはしない）
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.unminimize();
          }
        }
        _ => {}
      }
    }
    TrayIconEvent::DoubleClick { .. } => {
      if let Some(window) = app.get_webview_window("main") {
        // ダブルクリックでは確実に表示・フォーカス・最大化
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
      }
    }
    _ => {}
  }
}

fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
  match event.id().as_ref() {
    "show" => {
      if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
      }
    }
    "hide" => {
      if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
      }
    }
    "quit" => {
      std::process::exit(0);
    }
    _ => {}
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .on_window_event(|window, event| {
      match event {
        WindowEvent::CloseRequested { api, .. } => {
          // ウィンドウを閉じる代わりに最小化
          let _ = window.hide();
          api.prevent_close();
        }
        _ => {}
      }
    })
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // 通知プラグインを初期化
      app.handle().plugin(tauri_plugin_notification::init())?;
      
      // Get app handle before moving into async block
      let handle = app.handle().clone();
      
      // Get default icon before moving
      let icon = app.default_window_icon().unwrap().clone();
      
      // Initialize database
      tauri::async_runtime::block_on(async move {
        let db = Database::new(&handle)
          .await
          .expect("Failed to initialize database");
        
        // Initialize services
        let task_service = TaskService::new(db);
        
        // Add services to app state
        handle.manage(task_service);
      });
      
      // Create system tray menu
      let show_item = MenuItem::with_id(app, "show", "表示", true, None::<&str>)?;
      let hide_item = MenuItem::with_id(app, "hide", "非表示", true, None::<&str>)?;
      let quit_item = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;
      
      // Create system tray
      let _tray = TrayIconBuilder::new()
        .icon(icon)
        .title("TaskNag")
        .menu(&menu)
        .on_tray_icon_event(|tray, event| handle_tray_event(tray.app_handle(), event))
        .on_menu_event(|app, event| handle_menu_event(app, event))
        .build(app)?;
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::task_commands::create_task,
      commands::task_commands::get_tasks,
      commands::task_commands::get_task_by_id,
      commands::task_commands::update_task,
      commands::task_commands::delete_task,
      commands::task_commands::get_tasks_by_status,
      commands::task_commands::move_task,
      commands::task_commands::get_incomplete_task_count,
      commands::task_commands::update_tray_title,
      commands::task_commands::check_notifications,
      commands::task_commands::get_children,
      commands::task_commands::get_task_with_children,
      commands::task_commands::update_progress,
      commands::task_commands::calculate_and_update_progress,
      commands::task_commands::get_root_tasks,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

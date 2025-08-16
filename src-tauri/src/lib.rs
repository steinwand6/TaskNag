pub mod commands;
pub mod database;
pub mod error;
pub mod models;
pub mod services;

pub mod tests;

use database::Database;
use services::{TaskService, AgentService, PersonalityManager, BrowserActionService, NotificationService, ContextService};
use tauri::{
  AppHandle, Manager, WindowEvent, 
  tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
  menu::{Menu, MenuItem, MenuEvent}
};
use tauri_plugin_notification::NotificationExt;
use error::AppError;

// Helper function to check and fire notifications
async fn check_and_fire_notifications(
    notification_service: &NotificationService,
    app_handle: &AppHandle,
) -> Result<(), AppError> {
    use chrono::Local;
    let current_time = Local::now();
    let notifications = notification_service.check_notifications(current_time).await?;
    
    if !notifications.is_empty() {
        log::info!("発火する通知: {}件", notifications.len());
        
        for notification in notifications {
            log::info!("通知発火: {} (Level {})", notification.title, notification.level);
            
            // Fire the notification (includes browser actions)
            notification_service.fire_notification(&notification).await?;
            
            // Send Windows notification
            let title = match notification.notification_type.as_str() {
                "due_date_based" => format!("📅 期日通知"),
                "recurring" => format!("🔔 定期通知"),
                _ => format!("📋 通知"),
            };
            
            // Use Tauri notification plugin with sound for Windows
            #[cfg(target_os = "windows")]
            {
                app_handle.notification()
                    .builder()
                    .title(&title)
                    .body(&notification.title)
                    .sound("Default")  // Windows default notification sound
                    .show()
                    .map_err(|e| AppError::Internal(format!("通知送信エラー: {}", e)))?;
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                app_handle.notification()
                    .builder()
                    .title(&title)
                    .body(&notification.title)
                    .show()
                    .map_err(|e| AppError::Internal(format!("通知送信エラー: {}", e)))?;
            }
            
            // For level 3, maximize window
            if notification.level >= 3 {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        }
    }
    
    Ok(())
}

fn handle_tray_event(app: &AppHandle, event: TrayIconEvent) {
  match event {
    TrayIconEvent::Click { button, .. } => {
      if button == MouseButton::Left {
        if let Some(window) = app.get_webview_window("main") {
          // シングルクリックでは表示のみ（非表示にはしない）
          let _ = window.show();
          let _ = window.set_focus();
          let _ = window.unminimize();
        }
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
      if let WindowEvent::CloseRequested { api, .. } = event {
        // ウィンドウを閉じる代わりに最小化
        let _ = window.hide();
        api.prevent_close();
      }
    })
    .setup(|app| {
      // ログプラグインを常に有効化（デバッグ・リリース両方）
      app.handle().plugin(
        tauri_plugin_log::Builder::default()
          .level(log::LevelFilter::Debug)
          .targets([
            tauri_plugin_log::Target::new(
              tauri_plugin_log::TargetKind::LogDir { file_name: Some("tasknag".to_string()) }
            ),
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout)
          ])
          .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
          .build(),
      )?;
      
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
        let task_service = TaskService::new(db.clone());
        let mut agent_service = AgentService::new(db.pool.clone());
        let context_service = ContextService::new(db.pool.clone());
        
        // Load saved configuration if exists
        agent_service.load_saved_config().await.ok();
        
        let mut personality_manager_instance = PersonalityManager::new_with_db(Some(db.pool.clone()));
        personality_manager_instance.load_saved_personality().await.ok();
        let personality_manager = std::sync::Arc::new(std::sync::RwLock::new(personality_manager_instance));
        let browser_action_service = std::sync::Arc::new(BrowserActionService::new());
        let notification_service = NotificationService::with_browser_action_service(db.clone(), browser_action_service.clone());
        
        // Clone for notification scheduler
        let notification_service_clone = notification_service.clone();
        let app_handle_clone = handle.clone();
        
        // Start notification scheduler (15-minute intervals at :00, :15, :30, :45)
        tokio::spawn(async move {
            use chrono::{Local, Timelike};
            use std::time::Duration;
            
            // Calculate seconds until next quarter hour
            let seconds_until_next_quarter = || -> u64 {
                let now = Local::now();
                let current_minute = now.minute();
                let current_second = now.second();
                
                let next_quarter = match current_minute {
                    0..=14 => 15,
                    15..=29 => 30,
                    30..=44 => 45,
                    _ => 60,  // Next hour's :00
                };
                
                let minutes_to_wait = if next_quarter == 60 {
                    60 - current_minute
                } else {
                    next_quarter - current_minute
                };
                
                (minutes_to_wait * 60 - current_second) as u64
            };
            
            // Wait until next quarter hour
            let initial_wait = seconds_until_next_quarter();
            log::info!("通知スケジューラー: {}秒後に開始（次の15分区切り）", initial_wait);
            tokio::time::sleep(Duration::from_secs(initial_wait)).await;
            
            // Check notifications immediately at first quarter
            log::info!("通知スケジューラー: 初回チェック実行");
            if let Err(e) = check_and_fire_notifications(&notification_service_clone, &app_handle_clone).await {
                log::error!("通知チェックエラー: {}", e);
            }
            
            // Then check every 15 minutes
            let mut interval = tokio::time::interval(Duration::from_secs(900));
            interval.tick().await; // Skip first tick since we just checked
            
            loop {
                interval.tick().await;
                let now = Local::now();
                log::info!("通知チェック定期実行: {:02}:{:02}", now.hour(), now.minute());
                
                if let Err(e) = check_and_fire_notifications(&notification_service_clone, &app_handle_clone).await {
                    log::error!("通知チェックエラー: {}", e);
                }
            }
        });
        
        // Add services to app state
        handle.manage(task_service);
        handle.manage(agent_service);
        handle.manage(context_service);
        handle.manage(personality_manager);
        handle.manage(browser_action_service);
        handle.manage(notification_service);
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
        .on_menu_event(handle_menu_event)
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
      commands::task_commands::update_task_notification_settings,
      commands::task_commands::get_children,
      commands::task_commands::get_task_with_children,
      commands::task_commands::update_progress,
      commands::task_commands::calculate_and_update_progress,
      commands::task_commands::get_root_tasks,
      commands::task_commands::send_windows_notification,
      commands::task_commands::force_notification_check,
      commands::task_commands::test_notification_immediate,
      commands::tag_commands::get_all_tags,
      commands::tag_commands::get_tag_by_id,
      commands::tag_commands::create_tag,
      commands::tag_commands::update_tag,
      commands::tag_commands::delete_tag,
      commands::tag_commands::add_tag_to_task,
      commands::tag_commands::remove_tag_from_task,
      commands::tag_commands::get_tags_for_task,
      commands::log_commands::write_log,
      commands::log_commands::get_log_file_path,
      commands::log_commands::read_recent_logs,
      commands::agent_commands::test_ollama_connection,
      commands::agent_commands::list_ollama_models,
      commands::agent_commands::list_ollama_models_detailed,
      commands::agent_commands::get_agent_config,
      commands::agent_commands::get_model_preference,
      commands::agent_commands::get_model_preferences_for_available_models,
      commands::agent_commands::get_current_model,
      commands::agent_commands::set_current_model,
      commands::agent_commands::analyze_task_with_ai,
      commands::agent_commands::create_project_plan,
      commands::agent_commands::parse_natural_language_task,
      commands::agent_commands::chat_with_agent,
      commands::agent_commands::get_available_personalities,
      commands::agent_commands::set_ai_personality,
      commands::agent_commands::get_current_personality,
      commands::browser_commands::validate_url_command,
      commands::browser_commands::test_browser_action_command,
      commands::browser_commands::execute_browser_action_command,
      commands::browser_commands::execute_browser_actions_command,
      commands::browser_commands::test_url_command,
      commands::browser_commands::get_url_suggestions_command,
      commands::browser_commands::get_url_preview_command,
      commands::context_commands::get_temporal_context,
      commands::context_commands::get_task_context,
      commands::context_commands::get_basic_context,
      commands::context_commands::get_context_for_scope,
      commands::context_commands::get_context_as_prompt_variables,
      commands::prompt_commands::get_prompt_templates,
      commands::prompt_commands::get_prompt_template,
      commands::prompt_commands::generate_prompt,
      commands::prompt_commands::generate_task_consultation_prompt,
      commands::prompt_commands::generate_planning_prompt,
      commands::prompt_commands::generate_motivation_prompt,
      commands::prompt_commands::get_prompt_categories,
      commands::enhanced_agent_commands::chat_with_task_consultation,
      commands::enhanced_agent_commands::chat_with_planning_assistance,
      commands::enhanced_agent_commands::generate_motivation_boost,
      commands::enhanced_agent_commands::get_current_context,
      commands::enhanced_agent_commands::generate_context_aware_prompt,
      commands::enhanced_agent_commands::analyze_task_with_context,
      commands::enhanced_agent_commands::get_task_consultation_prompt,
      commands::enhanced_agent_commands::get_planning_prompt,
      commands::enhanced_agent_commands::get_motivation_prompt,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

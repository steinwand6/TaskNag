use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use tauri::AppHandle;

#[tauri::command]
pub async fn write_log(
    _app: AppHandle,
    level: String,
    message: String,
    data: Option<String>,
) -> Result<(), String> {
    // Use current working directory for logs during development
    let logs_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?
        .join("logs");
    std::fs::create_dir_all(&logs_dir)
        .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    
    // Create log file path (daily rotation)
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let log_file_path = logs_dir.join(format!("tasknag-{}.log", today));
    
    // Format log entry
    let timestamp = Utc::now().to_rfc3339();
    let data_str = data.unwrap_or_default();
    let log_entry = if data_str.is_empty() {
        format!("[{}] {}: {}\n", timestamp, level.to_uppercase(), message)
    } else {
        format!("[{}] {}: {} | Data: {}\n", timestamp, level.to_uppercase(), message, data_str)
    };
    
    // Write to log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;
    
    file.write_all(log_entry.as_bytes())
        .map_err(|e| format!("Failed to write to log file: {}", e))?;
    
    file.flush()
        .map_err(|e| format!("Failed to flush log file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_log_file_path(_app: AppHandle) -> Result<String, String> {
    // Use current working directory for logs during development
    let logs_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?
        .join("logs");
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let log_file_path = logs_dir.join(format!("tasknag-{}.log", today));
    
    Ok(log_file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn read_recent_logs(__app: AppHandle, lines: Option<usize>) -> Result<String, String> {
    // Use current working directory for logs during development
    let logs_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?
        .join("logs");
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let log_file_path = logs_dir.join(format!("tasknag-{}.log", today));
    
    if !log_file_path.exists() {
        return Ok("No logs found for today.".to_string());
    }
    
    let content = std::fs::read_to_string(&log_file_path)
        .map_err(|e| format!("Failed to read log file: {}", e))?;
    
    let lines_to_show = lines.unwrap_or(50);
    let log_lines: Vec<&str> = content.lines().collect();
    
    if log_lines.len() <= lines_to_show {
        Ok(content)
    } else {
        let recent_lines = &log_lines[log_lines.len() - lines_to_show..];
        Ok(recent_lines.join("
"))
    }
}
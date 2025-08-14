import { invoke } from '@tauri-apps/api/core';

export type LogLevel = 'info' | 'warn' | 'error' | 'debug';

export class LogService {
  /**
   * Write a log entry to the log file
   */
  static async writeLog(level: LogLevel, message: string, data?: any): Promise<void> {
    try {
      const dataStr = data ? JSON.stringify(data, null, 2) : undefined;
      await invoke('write_log', {
        level,
        message,
        data: dataStr,
      });
    } catch (error) {
      // Fallback to console if log service fails
      console.error('Failed to write to log file:', error);
      console[level](message, data);
    }
  }

  /**
   * Get the current log file path
   */
  static async getLogFilePath(): Promise<string> {
    return await invoke('get_log_file_path');
  }

  /**
   * Read recent log entries
   */
  static async getRecentLogs(lines: number = 50): Promise<string> {
    return await invoke('read_recent_logs', { lines });
  }

  /**
   * Convenience methods for different log levels
   */
  static async info(message: string, data?: any): Promise<void> {
    await this.writeLog('info', message, data);
  }

  static async warn(message: string, data?: any): Promise<void> {
    await this.writeLog('warn', message, data);
  }

  static async error(message: string, data?: any): Promise<void> {
    await this.writeLog('error', message, data);
  }

  static async debug(message: string, data?: any): Promise<void> {
    await this.writeLog('debug', message, data);
  }
}
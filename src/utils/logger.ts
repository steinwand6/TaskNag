/**
 * Logger utility for TaskNag application
 * Logs to both console and file for development convenience
 */

type LogLevel = 'info' | 'warn' | 'error' | 'debug';

interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  data?: any;
}

class Logger {
  private logEntries: LogEntry[] = [];
  private maxEntries = 1000; // Keep last 1000 log entries in memory

  private formatTimestamp(): string {
    return new Date().toISOString();
  }

  private createLogEntry(level: LogLevel, message: string, data?: any): LogEntry {
    return {
      timestamp: this.formatTimestamp(),
      level,
      message,
      data
    };
  }

  private addLogEntry(entry: LogEntry): void {
    this.logEntries.push(entry);
    
    // Keep only the last maxEntries
    if (this.logEntries.length > this.maxEntries) {
      this.logEntries = this.logEntries.slice(-this.maxEntries);
    }

    // Also log to console for immediate visibility
    const consoleMessage = `[${entry.timestamp}] ${entry.level.toUpperCase()}: ${entry.message}`;
    
    switch (entry.level) {
      case 'error':
        console.error(consoleMessage, entry.data);
        break;
      case 'warn':
        console.warn(consoleMessage, entry.data);
        break;
      case 'debug':
        console.debug(consoleMessage, entry.data);
        break;
      default:
        console.log(consoleMessage, entry.data);
    }
  }

  info(message: string, data?: any): void {
    this.addLogEntry(this.createLogEntry('info', message, data));
  }

  warn(message: string, data?: any): void {
    this.addLogEntry(this.createLogEntry('warn', message, data));
  }

  error(message: string, data?: any): void {
    this.addLogEntry(this.createLogEntry('error', message, data));
  }

  debug(message: string, data?: any): void {
    this.addLogEntry(this.createLogEntry('debug', message, data));
  }

  // Get recent logs for debugging
  getRecentLogs(count: number = 50): LogEntry[] {
    return this.logEntries.slice(-count);
  }

  // Get logs by level
  getLogsByLevel(level: LogLevel): LogEntry[] {
    return this.logEntries.filter(entry => entry.level === level);
  }

  // Export logs as text for external viewing
  exportLogsAsText(): string {
    return this.logEntries
      .map(entry => {
        const dataStr = entry.data ? ` | Data: ${JSON.stringify(entry.data)}` : '';
        return `[${entry.timestamp}] ${entry.level.toUpperCase()}: ${entry.message}${dataStr}`;
      })
      .join('\n');
  }

  // Clear all logs
  clear(): void {
    this.logEntries = [];
    console.clear();
  }
}

// Create singleton instance
export const logger = new Logger();

// Export convenience methods
export const log = {
  info: (message: string, data?: any) => logger.info(message, data),
  warn: (message: string, data?: any) => logger.warn(message, data),
  error: (message: string, data?: any) => logger.error(message, data),
  debug: (message: string, data?: any) => logger.debug(message, data),
  getRecent: (count?: number) => logger.getRecentLogs(count),
  export: () => logger.exportLogsAsText(),
  clear: () => logger.clear(),
};
import { listen } from '@tauri-apps/api/event';
import { LogService } from './logService';

export interface NotificationPayload {
  title: string;
  body: string;
  level?: number;
}

export class NotificationService {
  private static initialized = false;

  // 通知システムの初期化
  static async initialize(): Promise<void> {
    if (this.initialized) {
      return;
    }

    try {
      // Tauriイベントリスナーの設定（権限チェックは省略）
      await this.setupEventListeners();
      
      this.initialized = true;
      LogService.info('通知サービスが初期化されました');
    } catch (error) {
      LogService.error('通知サービス初期化エラー:', error);
    }
  }

  // Tauriイベントリスナーの設定
  private static async setupEventListeners(): Promise<void> {
    // 基本通知イベント
    await listen<NotificationPayload>('notification', (event) => {
      this.showNotification(event.payload);
    });

    // 音声付き通知イベント
    await listen<NotificationPayload>('sound_notification', (event) => {
      this.showNotification(event.payload, true);
    });

    LogService.info('通知イベントリスナーが設定されました');
  }

  // 通知の表示
  private static async showNotification(payload: NotificationPayload, withSound = false): Promise<void> {
    try {
      const { title, body, level } = payload;
      
      // レベルに応じた通知アイコンの設定
      let icon = '📋';
      if (level === 1) icon = '🔔';
      else if (level === 2) icon = '⚠️';
      else if (level === 3) icon = '🚨';

      // ブラウザのNotification APIを使用
      if ('Notification' in window) {
        // 権限チェック
        if (Notification.permission === 'granted') {
          const notification = new Notification(`${icon} ${title}`, {
            body,
            icon: '/tauri.svg', // アプリアイコン
            tag: 'tasknag-notification',
            requireInteraction: level === 3, // Level 3は手動で閉じる必要がある
          });

          // 通知クリック時の処理
          notification.onclick = () => {
            // フォーカスを戻す
            window.focus();
            notification.close();
          };

          // 自動で閉じる設定（Level 3以外）
          if (level !== 3) {
            setTimeout(() => {
              notification.close();
            }, 5000);
          }
        } else if (Notification.permission === 'default') {
          // 権限を要求
          const permission = await Notification.requestPermission();
          if (permission === 'granted') {
            // 権限が付与されたら再度通知を試行
            await this.showNotification(payload, withSound);
            return;
          }
        }
      }

      // 音声通知の場合（Level 2, 3）
      if (withSound) {
        this.playNotificationSound(level);
      }

      // Level 3の場合は追加のアクション
      if (level === 3) {
        this.handleHighPriorityNotification();
      }

      LogService.info(`通知を表示しました: ${title} - ${body}`);
    } catch (error) {
      LogService.error('通知表示エラー:', error);
    }
  }

  // 通知音の再生
  private static playNotificationSound(level = 1): void {
    try {
      // Web Audio APIを使用した通知音
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const oscillator = audioContext.createOscillator();
      const gainNode = audioContext.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(audioContext.destination);

      // レベルに応じた音の設定
      const frequency = level === 3 ? 800 : level === 2 ? 600 : 400;
      const duration = level === 3 ? 0.5 : 0.3;

      oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
      oscillator.type = 'sine';

      gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
      gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration);

      oscillator.start();
      oscillator.stop(audioContext.currentTime + duration);

      LogService.info(`通知音を再生しました (Level ${level})`);
    } catch (error) {
      LogService.error('通知音再生エラー:', error);
    }
  }

  // 高優先度通知の処理（Level 3）
  private static handleHighPriorityNotification(): void {
    // Level 3の通知は複数回繰り返し
    let count = 0;
    const maxCount = 3;
    
    const interval = setInterval(() => {
      count++;
      
      // 短いビープ音
      this.playNotificationSound(3);
      
      if (count >= maxCount) {
        clearInterval(interval);
      }
    }, 1000);

    LogService.info('高優先度通知処理を実行しました');
  }

  // 手動通知テスト
  static async testNotification(): Promise<void> {
    await this.showNotification({
      title: 'テスト通知',
      body: 'TaskNag通知システムのテストです',
      level: 2
    }, true);
  }

  // 即座通知テスト（実際の通知設定を使用）
  static async testNotificationImmediate(): Promise<void> {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('test_notification_immediate');
      LogService.info('即座通知テストを実行しました');
    } catch (error) {
      LogService.error('即座通知テストエラー:', error);
    }
  }

  // Windows通知送信
  static async sendWindowsNotification(title: string, body: string, level: number = 1): Promise<void> {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('send_windows_notification', { title, body, level });
      LogService.info(`Windows通知を送信: ${title} - ${body} (Level ${level})`);
    } catch (error) {
      LogService.error('Windows通知送信エラー:', error);
    }
  }
}
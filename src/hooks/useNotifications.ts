import { useEffect, useCallback } from 'react';
import { TaskService } from '../services/taskService';
import { LogService } from '../services/logService';
import { TaskNotification } from '../types/Task';

export const useNotifications = () => {
  const checkNotifications = useCallback(async (): Promise<TaskNotification[]> => {
    try {
      const notifications = await TaskService.checkNotifications();
      console.log('通知チェック完了:', notifications);
      
      // 通知があった場合は表示
      if (notifications.length > 0) {
        for (const notification of notifications) {
          // システム通知を表示
          if ('Notification' in window && Notification.permission === 'granted') {
            const browserNotification = new Notification(`🔔 ${notification.title}`, {
              body: `レベル: ${notification.level} | ${notification.notificationType}`,
              icon: '/tauri.svg',
              tag: `tasknag-${notification.taskId}`,
            });

            // 通知クリック時にフォーカス
            browserNotification.onclick = () => {
              window.focus();
              browserNotification.close();
            };

            // 5秒後に自動で閉じる
            setTimeout(() => {
              browserNotification.close();
            }, 5000);
          }
        }
        
        LogService.info(`${notifications.length}件の通知を表示しました`);
      }
      
      return notifications;
    } catch (error) {
      LogService.error('通知チェックエラー', error);
      return [];
    }
  }, []);

  // 定期的な通知チェック（1分間隔）
  useEffect(() => {
    const initializeNotifications = async () => {
      // 通知権限の要求
      if ('Notification' in window && Notification.permission === 'default') {
        try {
          const permission = await Notification.requestPermission();
          LogService.info(`通知権限: ${permission}`);
        } catch (error) {
          LogService.error('通知権限要求エラー:', error);
        }
      }

      // 初回チェック
      checkNotifications();
    };

    initializeNotifications();

    // 定期チェックの設定
    const interval = setInterval(() => {
      checkNotifications();
    }, 1 * 60 * 1000); // 1分

    return () => clearInterval(interval);
  }, [checkNotifications]);

  return {
    checkNotifications,
  };
};
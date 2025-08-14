import { useEffect, useCallback } from 'react';
import { TaskService } from '../services/taskService';
import { LogService } from '../services/logService';
import { TaskNotification } from '../types/Task';

export const useNotifications = () => {
  const checkNotifications = useCallback(async (): Promise<TaskNotification[]> => {
    try {
      const notifications = await TaskService.checkNotifications();
      console.log('通知チェック完了:', notifications);
      return notifications;
    } catch (error) {
      LogService.error('通知チェックエラー', error);
      return [];
    }
  }, []);

  // 定期的な通知チェック（5分間隔）
  useEffect(() => {
    // 初回チェック
    checkNotifications();

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
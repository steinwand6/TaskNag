import { useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LogService } from '../services/logService';
import { TaskNotification } from '../types/Task';

export const useNotifications = () => {
  const checkNotifications = useCallback(async (): Promise<TaskNotification[]> => {
    try {
      const notifications = await invoke('force_notification_check') as TaskNotification[];
      console.log('通知チェック完了:', notifications);
      
      // NotificationServiceが通知表示とブラウザアクション実行を処理
      if (notifications.length > 0) {
        LogService.info(`${notifications.length}件の通知が発火しました`);
      }
      
      return notifications;
    } catch (error) {
      LogService.error('通知チェックエラー', error);
      return [];
    }
  }, []);

  // 通知権限の要求のみ（通知チェックは完全にバックエンドで実行）
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

      // 初回チェックも削除 - 全てバックエンドの15分スケジューラーに任せる
      // checkNotifications();
    };

    initializeNotifications();

    // フロントエンドでの通知チェックは完全に削除（バックエンドで15分間隔で自動実行）
  }, []);

  return {
    checkNotifications,
  };
};
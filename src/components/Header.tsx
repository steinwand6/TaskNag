import React, { useState, useEffect } from 'react';
import { NotificationService } from '../services/notificationService';

interface HeaderProps {
  isLoading: boolean;
  onNewTask: () => void;
  onRefresh: () => void;
  showDone: boolean;
  onToggleDone: () => void;
  onManageTags?: () => void;
  onToggleFilters?: () => void;
  showFilters?: boolean;
  hasActiveFilters?: boolean;
}

export const Header: React.FC<HeaderProps> = ({ 
  isLoading, 
  onNewTask, 
  onRefresh, 
  showDone, 
  onToggleDone, 
  onManageTags, 
  onToggleFilters, 
  showFilters = false, 
  hasActiveFilters = false
}) => {
  const [currentTime, setCurrentTime] = useState(new Date());
  const [showTimeInfo, setShowTimeInfo] = useState(false);

  // 現在時刻を1秒ごとに更新
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  const handleTestNotification = async () => {
    // 両方の通知をテスト
    await NotificationService.testNotification(); // ブラウザ通知
    await NotificationService.testNotificationImmediate(); // Windows通知（設定済みタスク）
    await NotificationService.sendWindowsNotification("🔔 テスト通知", "Windows通知システムのテストです", 2); // 直接Windows通知
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('ja-JP', { 
      hour: '2-digit', 
      minute: '2-digit', 
      second: '2-digit' 
    });
  };
  return (
    <header className="bg-white shadow-sm border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center py-4">
          <div className="flex items-center">
            <h1 className="text-2xl font-bold text-gray-900">
              TaskNag 🗣️
            </h1>
            <p className="ml-3 text-sm text-gray-500">
              口うるさいタスク管理
            </p>
          </div>
          <div className="flex items-center space-x-4">
            {onToggleFilters && (
              <button 
                onClick={onToggleFilters}
                className={`px-3 py-2 text-sm rounded-md transition-colors relative ${
                  showFilters 
                    ? 'bg-blue-100 text-blue-800 border border-blue-300' 
                    : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
                }`}
                disabled={isLoading}
              >
                🔍 フィルタ
                {hasActiveFilters && (
                  <span className="absolute -top-1 -right-1 w-2 h-2 bg-red-500 rounded-full"></span>
                )}
              </button>
            )}
            {onManageTags && (
              <button 
                onClick={onManageTags}
                className="px-3 py-2 text-sm rounded-md bg-purple-100 text-purple-800 border border-purple-300 hover:bg-purple-200 transition-colors"
                disabled={isLoading}
              >
                🏷️ タグ管理
              </button>
            )}
            <button 
              onClick={onToggleDone}
              className={`px-3 py-2 text-sm rounded-md transition-colors ${
                showDone 
                  ? 'bg-green-100 text-green-800 border border-green-300' 
                  : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
              }`}
              disabled={isLoading}
            >
              {showDone ? '✅ DONE表示中' : '✅ DONE'}
            </button>
            <button 
              onClick={onNewTask}
              className="btn-primary"
              disabled={isLoading}
            >
              + 新規タスク
            </button>
            <button 
              className="btn-secondary"
              onClick={onRefresh}
              disabled={isLoading}
            >
              {isLoading ? '⏳' : '🔄'} 更新
            </button>
            <button 
              className="px-3 py-2 text-sm rounded-md bg-blue-100 text-blue-800 border border-blue-300 hover:bg-blue-200 transition-colors"
              onClick={() => setShowTimeInfo(!showTimeInfo)}
              disabled={isLoading}
              title="現在時刻と通知設定ヘルプ"
            >
              🕐 時刻情報
            </button>
            <button 
              className="px-3 py-2 text-sm rounded-md bg-yellow-100 text-yellow-800 border border-yellow-300 hover:bg-yellow-200 transition-colors"
              onClick={handleTestNotification}
              disabled={isLoading}
              title="通知システムのテスト"
            >
              🔔 通知テスト
            </button>
          </div>
        </div>
        
        {/* 時刻情報パネル */}
        {showTimeInfo && (
          <div className="border-t border-gray-200 bg-blue-50 px-4 py-3">
            <div className="max-w-7xl mx-auto">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-6">
                  <div className="text-sm">
                    <span className="font-semibold text-blue-900">現在時刻:</span>{' '}
                    <span className="text-blue-800 font-mono text-base">
                      {formatTime(currentTime)}
                    </span>
                  </div>
                  <div className="text-xs text-blue-700">
                    通知は設定時刻の±30秒以内で作動します
                  </div>
                </div>
                <div className="text-xs text-blue-600">
                  💡 テスト用に現在時刻付近（例：{formatTime(new Date(currentTime.getTime() + 60000))}）で通知時刻を設定してください
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </header>
  );
};
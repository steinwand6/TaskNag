import React, { useState, useEffect } from 'react';
import { TaskNotificationSettings } from '../types/Task';
import { LogService } from '../services/logService';

interface NotificationSettingsProps {
  settings: TaskNotificationSettings;
  onChange: (settings: TaskNotificationSettings) => void;
  hasDueDate: boolean;
}

export const NotificationSettings: React.FC<NotificationSettingsProps> = ({
  settings,
  onChange,
  hasDueDate,
}) => {
  const [localSettings, setLocalSettings] = useState<TaskNotificationSettings>(settings);

  useEffect(() => {
    setLocalSettings(settings);
  }, [settings]);

  const handleTypeChange = (type: TaskNotificationSettings['notificationType']) => {
    const newSettings: TaskNotificationSettings = {
      ...localSettings,
      notificationType: type,
      // デフォルト値を設定
      daysBefore: type === 'due_date_based' ? 3 : undefined,
      notificationTime: type !== 'none' ? '09:00' : undefined,
      daysOfWeek: type === 'recurring' ? [1, 2, 3, 4, 5] : undefined, // 平日デフォルト
    };
    setLocalSettings(newSettings);
    onChange(newSettings);
  };

  const handleDaysBeforeChange = (days: number) => {
    const newSettings = { ...localSettings, daysBefore: days };
    setLocalSettings(newSettings);
    onChange(newSettings);
  };

  const handleTimeChange = (time: string) => {
    const newSettings = { ...localSettings, notificationTime: time };
    setLocalSettings(newSettings);
    onChange(newSettings);
  };

  const handleDayToggle = (day: number) => {
    LogService.info('通知設定', `曜日ボタンクリック: ${day}`);
    const currentDays = localSettings.daysOfWeek || [];
    const newDays = currentDays.includes(day)
      ? currentDays.filter(d => d !== day)
      : [...currentDays, day].sort();
    
    const newSettings = { ...localSettings, daysOfWeek: newDays };
    setLocalSettings(newSettings);
    onChange(newSettings);
  };

  const handleLevelChange = (level: 1 | 2 | 3) => {
    LogService.info('通知設定', `通知レベル変更: ${level}`);
    const newSettings = { ...localSettings, level };
    setLocalSettings(newSettings);
    onChange(newSettings);
  };

  const dayNames = ['日', '月', '火', '水', '木', '金', '土'];

  return (
    <div className="space-y-4">
      {/* 通知タイプ選択 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          通知設定
        </label>
        <select
          value={localSettings.notificationType}
          onChange={(e) => handleTypeChange(e.target.value as TaskNotificationSettings['notificationType'])}
          onClick={(e) => e.stopPropagation()}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="none">通知なし</option>
          {hasDueDate && (
            <option value="due_date_based">期日ベース通知</option>
          )}
          <option value="recurring">定期通知</option>
        </select>
      </div>

      {/* 期日ベース設定 */}
      {localSettings.notificationType === 'due_date_based' && (
        <div className="space-y-3 pl-4 border-l-2 border-blue-300">
          <div className="flex items-center gap-2">
            <label className="text-sm text-gray-600">期日</label>
            <input
              type="number"
              min="1"
              max="30"
              value={localSettings.daysBefore || 3}
              onChange={(e) => handleDaysBeforeChange(parseInt(e.target.value))}
              onClick={(e) => e.stopPropagation()}
              className="w-16 px-2 py-1 border border-gray-300 rounded"
            />
            <span className="text-sm text-gray-600">日前から</span>
          </div>
          
          <div className="flex items-center gap-2">
            <label className="text-sm text-gray-600">通知時刻</label>
            <input
              type="time"
              value={localSettings.notificationTime || '09:00'}
              onChange={(e) => handleTimeChange(e.target.value)}
              onClick={(e) => e.stopPropagation()}
              className="px-2 py-1 border border-gray-300 rounded"
            />
          </div>
        </div>
      )}

      {/* 定期通知設定 */}
      {localSettings.notificationType === 'recurring' && (
        <div className="space-y-3 pl-4 border-l-2 border-green-300">
          <div>
            <label className="text-sm text-gray-600 block mb-2">曜日選択</label>
            <div className="flex gap-1">
              {dayNames.map((day, index) => (
                <button
                  key={index}
                  type="button"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    LogService.info('通知設定', `曜日${index}ボタンクリック`);
                    handleDayToggle(index);
                  }}
                  className={`w-8 h-8 rounded-full text-xs font-medium transition-colors ${
                    localSettings.daysOfWeek?.includes(index)
                      ? 'bg-blue-500 text-white'
                      : 'bg-gray-200 text-gray-600 hover:bg-gray-300'
                  }`}
                >
                  {day}
                </button>
              ))}
            </div>
          </div>
          
          <div className="flex items-center gap-2">
            <label className="text-sm text-gray-600">通知時刻</label>
            <input
              type="time"
              value={localSettings.notificationTime || '09:00'}
              onChange={(e) => handleTimeChange(e.target.value)}
              onClick={(e) => e.stopPropagation()}
              className="px-2 py-1 border border-gray-300 rounded"
            />
          </div>
        </div>
      )}

      {/* 通知レベル設定 */}
      {localSettings.notificationType !== 'none' && (
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            通知レベル
          </label>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                LogService.info('通知設定', 'レベル1ボタンクリック');
                handleLevelChange(1);
              }}
              className={`px-3 py-1 rounded text-sm ${
                localSettings.level === 1
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Lv1: 通知のみ
            </button>
            <button
              type="button"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                LogService.info('通知設定', 'レベル2ボタンクリック');
                handleLevelChange(2);
              }}
              className={`px-3 py-1 rounded text-sm ${
                localSettings.level === 2
                  ? 'bg-yellow-500 text-white'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Lv2: 通知+音
            </button>
            <button
              type="button"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                LogService.info('通知設定', 'レベル3ボタンクリック');
                handleLevelChange(3);
              }}
              className={`px-3 py-1 rounded text-sm ${
                localSettings.level === 3
                  ? 'bg-red-500 text-white'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Lv3: 最大化+通知+音
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
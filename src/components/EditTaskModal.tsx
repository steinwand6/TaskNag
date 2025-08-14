import React from 'react';
import { Task, TaskStatus, Priority, TaskNotificationSettings } from '../types/Task';
import { NotificationSettings } from './NotificationSettings';
import { useTaskStore } from '../stores/taskStore';
import { DEFAULT_TASK_STATUS, STATUS_OPTIONS, PRIORITY_OPTIONS } from '../constants';
import { LogService } from '../services/logService';

interface EditTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  task: Task | null;
}

export const EditTaskModal: React.FC<EditTaskModalProps> = ({ isOpen, onClose, task }) => {
  // デバッグ用: onCloseが呼ばれた時のログ
  const handleClose = () => {
    LogService.info('モーダル', 'EditTaskModal: onCloseが呼ばれました');
    onClose();
  };
  const { updateTask } = useTaskStore();
  const [formData, setFormData] = React.useState({
    title: '',
    description: '',
    priority: 'medium' as Priority,
    status: DEFAULT_TASK_STATUS,
    dueDate: '',
    notificationSettings: {
      notificationType: 'none',
      level: 1,
    } as TaskNotificationSettings,
  });
  
  // Initialize form data when task changes
  React.useEffect(() => {
    if (task) {
      setFormData({
        title: task.title,
        description: task.description || '',
        priority: task.priority,
        status: task.status,
        dueDate: task.dueDate ? task.dueDate.toISOString().split('T')[0] : '',
        notificationSettings: task.notificationSettings || {
          notificationType: 'none',
          level: 1,
        },
      });
    }
  }, [task]);
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    LogService.info('モーダル', 'EditTaskModal: handleSubmitが呼ばれました');
    if (!formData.title.trim() || !task) return;
    
    updateTask(task.id, {
      title: formData.title,
      description: formData.description || undefined,
      priority: formData.priority,
      status: formData.status,
      dueDate: formData.dueDate ? new Date(formData.dueDate) : undefined,
      notificationSettings: formData.notificationSettings,
    });
    
    handleClose();
  };

  const handleNotificationChange = (settings: TaskNotificationSettings) => {
    LogService.info('モーダル', 'EditTaskModal: 通知設定が変更されました', JSON.stringify(settings));
    setFormData({ ...formData, notificationSettings: settings });
  };
  
  if (!isOpen || !task) return null;
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg w-full max-w-md max-h-[90vh] flex flex-col overflow-hidden">
        <div className="p-6 overflow-y-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold">タスクを編集</h2>
          <button
            onClick={handleClose}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              タスク名 *
            </label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="タスクのタイトルを入力..."
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              説明
            </label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
              placeholder="タスクの詳細説明（任意）..."
            />
          </div>
          
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                優先度
              </label>
              <select
                value={formData.priority}
                onChange={(e) => setFormData({ ...formData, priority: e.target.value as Priority })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {PRIORITY_OPTIONS.map(({ value, label }) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ステータス
              </label>
              <select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value as TaskStatus })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {STATUS_OPTIONS.map(({ value, label }) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              期限
            </label>
            <input
              type="date"
              value={formData.dueDate}
              onChange={(e) => setFormData({ ...formData, dueDate: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          {/* 通知設定セクション */}
          <div className="border-t pt-4">
            <NotificationSettings
              settings={formData.notificationSettings}
              onChange={handleNotificationChange}
              hasDueDate={!!formData.dueDate}
            />
          </div>
          
          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={handleClose}
              className="px-4 py-2 text-gray-600 hover:text-gray-800"
            >
              キャンセル
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              更新
            </button>
          </div>
        </form>
        </div>
      </div>
    </div>
  );
};
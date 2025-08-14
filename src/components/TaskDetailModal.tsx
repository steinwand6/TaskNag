import React from 'react';
import { Task } from '../types/Task';
import { SubTaskList } from './SubTaskList';
import { ProgressBar } from './ProgressBar';
import { XMarkIcon } from '@heroicons/react/24/outline';

interface TaskDetailModalProps {
  task: Task | null;
  isOpen: boolean;
  onClose: () => void;
  onTaskUpdate?: (task: Task) => void;
}

export const TaskDetailModal: React.FC<TaskDetailModalProps> = ({
  task,
  isOpen,
  onClose,
  onTaskUpdate
}) => {
  if (!isOpen || !task) return null;

  const formatDate = (date?: Date) => {
    if (!date) return '未設定';
    return new Intl.DateTimeFormat('ja-JP', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  };

  const getPriorityDisplay = (priority: Task['priority']) => {
    const priorityMap = {
      required: '🚨 必須',
      high: '🔴 高',
      medium: '🟡 中',
      low: '🟢 低'
    };
    return priorityMap[priority] || priority;
  };

  const getStatusDisplay = (status: Task['status']) => {
    const statusMap = {
      inbox: '📥 INBOX',
      todo: '📋 TODO',
      in_progress: '⚡ 実行中',
      done: '✅ 完了'
    };
    return statusMap[status] || status;
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-hidden">
        {/* ヘッダー */}
        <div className="flex items-center justify-between p-6 border-b">
          <h2 className="text-xl font-semibold text-gray-900">タスク詳細</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 transition-colors"
          >
            <XMarkIcon className="w-6 h-6" />
          </button>
        </div>

        {/* コンテンツ */}
        <div className="p-6 overflow-y-auto max-h-[calc(90vh-140px)]">
          {/* タスク基本情報 */}
          <div className="space-y-4 mb-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                {task.title}
              </h3>
              {task.description && (
                <p className="text-gray-600 whitespace-pre-wrap">
                  {task.description}
                </p>
              )}
            </div>

            {/* メタ情報 */}
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <span className="text-gray-500">ステータス:</span>
                <div className="font-medium">{getStatusDisplay(task.status)}</div>
              </div>
              <div>
                <span className="text-gray-500">優先度:</span>
                <div className="font-medium">{getPriorityDisplay(task.priority)}</div>
              </div>
              <div>
                <span className="text-gray-500">期限:</span>
                <div className="font-medium">{formatDate(task.dueDate)}</div>
              </div>
              <div>
                <span className="text-gray-500">進捗:</span>
                <div className="font-medium">{task.progress || 0}%</div>
              </div>
            </div>

            {/* 進捗バー */}
            {task.progress !== undefined && (
              <ProgressBar progress={task.progress} />
            )}
          </div>

          {/* 子タスク */}
          <div className="border-t pt-6">
            <SubTaskList
              parentTask={task}
              onTaskUpdate={onTaskUpdate}
            />
          </div>

          {/* タイムスタンプ */}
          <div className="border-t pt-4 mt-6 text-xs text-gray-500 space-y-1">
            <div>作成日: {formatDate(task.createdAt)}</div>
            <div>更新日: {formatDate(task.updatedAt)}</div>
            {task.completedAt && (
              <div>完了日: {formatDate(task.completedAt)}</div>
            )}
          </div>
        </div>

        {/* フッター */}
        <div className="flex justify-end gap-3 p-6 border-t bg-gray-50">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
          >
            閉じる
          </button>
        </div>
      </div>
    </div>
  );
};
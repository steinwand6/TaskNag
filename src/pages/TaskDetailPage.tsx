import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Task } from '../types/Task';
import { TaskService } from '../services/taskService';
import { SubTaskList } from '../components/SubTaskList';
import { ArrowLeftIcon } from '@heroicons/react/24/outline';

export const TaskDetailPage: React.FC = () => {
  const { taskId } = useParams<{ taskId: string }>();
  const navigate = useNavigate();
  const [task, setTask] = useState<Task | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // デバッグ用
  console.log('TaskDetailPage rendered with taskId:', taskId);

  useEffect(() => {
    if (!taskId) {
      setError('タスクIDが見つかりません');
      setLoading(false);
      return;
    }

    loadTask();
  }, [taskId]);

  const loadTask = async () => {
    if (!taskId) return;
    
    try {
      setLoading(true);
      // Fallback to getTaskById if getTaskWithChildren doesn't exist
      const taskData = await TaskService.getTaskById(taskId);
      setTask(taskData);
      console.log('Task loaded:', taskData);
    } catch (err) {
      console.error('Failed to load task:', err);
      setError(`タスクの読み込みに失敗しました: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleTaskUpdate = (updatedTask: Task) => {
    console.log('TaskDetailPage: handleTaskUpdate called with:', updatedTask);
    setTask(updatedTask);
  };

  const formatDate = (date?: Date | string) => {
    if (!date) return '未設定';
    
    try {
      const dateObj = typeof date === 'string' ? new Date(date) : date;
      if (isNaN(dateObj.getTime())) return '無効な日付';
      
      return new Intl.DateTimeFormat('ja-JP', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      }).format(dateObj);
    } catch (error) {
      console.error('Date formatting error:', error, date);
      return '日付エラー';
    }
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

  console.log('Render state:', { loading, error, task, taskId });

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-gray-600">読み込み中... (TaskId: {taskId})</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-red-600">{error}</div>
        <button onClick={() => navigate('/')} className="ml-4 px-4 py-2 bg-blue-500 text-white rounded">
          戻る
        </button>
      </div>
    );
  }

  if (!task) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-gray-600">タスクが見つかりません (TaskId: {taskId})</div>
        <button onClick={() => navigate('/')} className="ml-4 px-4 py-2 bg-blue-500 text-white rounded">
          戻る
        </button>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ヘッダー */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center py-4">
            <button
              onClick={() => navigate('/')}
              className="flex items-center gap-2 text-gray-600 hover:text-gray-800 transition-colors"
            >
              <ArrowLeftIcon className="w-5 h-5" />
              戻る
            </button>
            <h1 className="ml-4 text-xl font-semibold text-gray-900">
              タスク詳細
            </h1>
          </div>
        </div>
      </header>

      {/* コンテンツ */}
      <main className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
          {/* タスク基本情報 */}
          <div className="p-6 space-y-6">
            <div>
              <h2 className="text-2xl font-semibold text-gray-900 mb-3">
                {task.title}
              </h2>
              {task.description && (
                <div className="bg-gray-50 rounded-lg p-4">
                  <h3 className="text-sm font-medium text-gray-700 mb-2">説明</h3>
                  <p className="text-gray-700 whitespace-pre-wrap">
                    {task.description}
                  </p>
                </div>
              )}
            </div>

            {/* メタ情報 */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="bg-gray-50 rounded-lg p-4">
                <div className="text-sm text-gray-500 mb-1">ステータス</div>
                <div className="font-semibold text-gray-900">
                  {getStatusDisplay(task.status)}
                </div>
              </div>
              <div className="bg-gray-50 rounded-lg p-4">
                <div className="text-sm text-gray-500 mb-1">優先度</div>
                <div className="font-semibold text-gray-900">
                  {getPriorityDisplay(task.priority)}
                </div>
              </div>
              <div className="bg-gray-50 rounded-lg p-4">
                <div className="text-sm text-gray-500 mb-1">期限</div>
                <div className="font-semibold text-gray-900">
                  {formatDate(task.dueDate)}
                </div>
              </div>
            </div>
            
            {/* 通知設定表示 */}
            {task.notificationSettings && task.notificationSettings.notificationType !== 'none' && (
              <div className="bg-blue-50 rounded-lg p-4">
                <h3 className="text-sm font-medium text-gray-700 mb-2">通知設定</h3>
                <div className="text-gray-900">
                  {task.notificationSettings.notificationType === 'due_date_based' ? (
                    <span>📅 期日{task.notificationSettings.daysBefore}日前 {task.notificationSettings.notificationTime}に通知</span>
                  ) : task.notificationSettings.notificationType === 'recurring' ? (
                    <span>🔔 定期通知 {task.notificationSettings.notificationTime}</span>
                  ) : null}
                  <span className="ml-2 text-sm text-gray-600">
                    (Level {task.notificationSettings.level})
                  </span>
                </div>
              </div>
            )}
          </div>

          {/* 進捗・子タスク管理 */}
          <div className="border-t border-gray-200 p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">進捗・子タスク管理</h3>
            <SubTaskList
              parentTask={task}
              onTaskUpdate={(updatedTask) => {
                console.log('TaskDetailPage: About to call handleTaskUpdate with:', updatedTask);
                handleTaskUpdate(updatedTask);
              }}
            />
          </div>

          {/* タイムスタンプ */}
          <div className="border-t border-gray-200 px-6 py-4 bg-gray-50">
            <h3 className="text-sm font-medium text-gray-700 mb-3">タイムスタンプ</h3>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-600">
              <div>
                <span className="font-medium">作成日:</span>
                <div>{formatDate(task.createdAt)}</div>
              </div>
              <div>
                <span className="font-medium">更新日:</span>
                <div>{formatDate(task.updatedAt)}</div>
              </div>
              {task.completedAt && (
                <div>
                  <span className="font-medium">完了日:</span>
                  <div>{formatDate(task.completedAt)}</div>
                </div>
              )}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};
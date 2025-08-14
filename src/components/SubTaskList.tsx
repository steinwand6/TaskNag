import React, { useState, useEffect, useCallback } from 'react';
import { Task } from '../types/Task';
import { TaskService } from '../services/taskService';
import { ProgressBar } from './ProgressBar';

interface SubTaskListProps {
  parentTask: Task;
  onTaskUpdate?: (task: Task) => void;
}

export const SubTaskList: React.FC<SubTaskListProps> = ({ parentTask, onTaskUpdate }) => {
  const [children, setChildren] = useState<Task[]>([]);
  const [isExpanded, setIsExpanded] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [editingSubtask, setEditingSubtask] = useState<Task | null>(null);
  const [newSubtask, setNewSubtask] = useState({
    title: '',
    description: '',
    dueDate: '',
    notificationType: 'none' as 'none' | 'due_date_based' | 'recurring',
    daysBefore: 1,
    notificationTime: '09:00',
  });

  // ローカルでの進捗率計算
  const calculateProgress = (childTasks: Task[]) => {
    if (childTasks.length === 0) return 0;
    const completedCount = childTasks.filter(child => child.status === 'done').length;
    return Math.round((completedCount / childTasks.length) * 100);
  };

  const loadChildren = useCallback(async () => {
    if (!parentTask.id) return;
    
    setIsLoading(true);
    try {
      const childTasks = await TaskService.getChildren(parentTask.id);
      setChildren(childTasks);
    } catch (error) {
      console.error('Failed to load child tasks:', error);
    } finally {
      setIsLoading(false);
    }
  }, [parentTask.id]);

  useEffect(() => {
    if (isExpanded) {
      loadChildren();
    }
  }, [isExpanded, parentTask.id, loadChildren]);

  const handleDelete = async (taskId: string) => {
    if (!confirm('この子タスクを削除しますか？')) return;
    
    try {
      await TaskService.deleteTask(taskId);
      
      // 子タスクリストから削除
      const updatedChildren = children.filter(child => child.id !== taskId);
      setChildren(updatedChildren);
      
      // 親タスクの進捗率を即座に更新
      if (onTaskUpdate) {
        const newProgress = calculateProgress(updatedChildren);
        onTaskUpdate({
          ...parentTask,
          progress: newProgress
        });
        
        // バックエンドでも更新（非同期）
        if (parentTask.id) {
          TaskService.calculateAndUpdateProgress(parentTask.id).catch(console.error);
        }
      }
    } catch (error) {
      console.error('Failed to delete subtask:', error);
      alert('子タスクの削除に失敗しました');
    }
  };

  const handleEditSubtask = (subtask: Task) => {
    setEditingSubtask(subtask);
    setNewSubtask({
      title: subtask.title,
      description: subtask.description || '',
      dueDate: subtask.dueDate ? new Date(subtask.dueDate).toISOString().split('T')[0] : '',
      notificationType: subtask.notificationSettings?.notificationType || 'none',
      daysBefore: subtask.notificationSettings?.daysBefore || 1,
      notificationTime: subtask.notificationSettings?.notificationTime || '09:00',
    });
    setShowAddDialog(true);
  };

  const handleSaveSubtask = async () => {
    if (!newSubtask.title.trim()) {
      alert('タイトルを入力してください');
      return;
    }

    try {
      // 通知設定の準備
      let notificationSettings = undefined;
      if (newSubtask.notificationType !== 'none' && newSubtask.dueDate) {
        notificationSettings = {
          notificationType: newSubtask.notificationType as 'due_date_based' | 'recurring',
          daysBefore: newSubtask.daysBefore,
          notificationTime: newSubtask.notificationTime,
          daysOfWeek: [],
          level: 3 as 1 | 2 | 3,
        };
      }

      if (editingSubtask) {
        // 既存の子タスクを更新
        const updatedTask = await TaskService.updateTask(editingSubtask.id, {
          ...editingSubtask,
          title: newSubtask.title.trim(),
          description: newSubtask.description.trim(),
          dueDate: newSubtask.dueDate ? new Date(newSubtask.dueDate) : undefined,
          notificationSettings,
        });

        const updatedChildren = children.map(child =>
          child.id === editingSubtask.id ? updatedTask : child
        );
        setChildren(updatedChildren);
      } else {
        // 新規子タスクを作成
        const createdSubtask = await TaskService.createTask({
          title: newSubtask.title.trim(),
          description: newSubtask.description.trim(),
          status: 'todo',
          parentId: parentTask.id,
          dueDate: newSubtask.dueDate ? new Date(newSubtask.dueDate) : undefined,
          notificationSettings,
        });

        const updatedChildren = [...children, createdSubtask];
        setChildren(updatedChildren);

        // 親タスクの進捗率を即座に更新
        if (onTaskUpdate) {
          const newProgress = calculateProgress(updatedChildren);
          onTaskUpdate({
            ...parentTask,
            progress: newProgress
          });

          // バックエンドでも更新（非同期）
          if (parentTask.id) {
            TaskService.calculateAndUpdateProgress(parentTask.id).catch(console.error);
          }
        }
      }

      // ダイアログをリセット
      setShowAddDialog(false);
      setEditingSubtask(null);
      setNewSubtask({
        title: '',
        description: '',
        dueDate: '',
        notificationType: 'none',
        daysBefore: 1,
        notificationTime: '09:00',
      });
    } catch (error) {
      console.error('Failed to save subtask:', error);
      alert('子タスクの保存に失敗しました');
    }
  };

  const handleStatusChange = async (taskId: string, isDone: boolean) => {
    try {
      const newStatus = isDone ? 'done' : 'todo';
      const updatedTask = await TaskService.moveTask(taskId, newStatus as any);
      
      // ステータス変更時に進捗率も自動更新（100% or 0%）
      await TaskService.updateProgress(taskId, isDone ? 100 : 0);
      
      // 子タスクリストを更新
      const updatedChildren = children.map(child => 
        child.id === taskId ? { ...updatedTask, progress: isDone ? 100 : 0 } : child
      );
      setChildren(updatedChildren);
      
      // 親タスクの進捗率を即座に更新
      if (onTaskUpdate) {
        const newProgress = calculateProgress(updatedChildren);
        console.log('SubTaskList handleStatusChange: Updating parent progress to:', newProgress, 'from children:', updatedChildren.map(c => ({id: c.id, status: c.status})));
        onTaskUpdate({
          ...parentTask,
          progress: newProgress
        });
        
        // バックエンドでも更新（非同期）
        if (parentTask.id) {
          TaskService.calculateAndUpdateProgress(parentTask.id).catch(console.error);
        }
      }
    } catch (error) {
      console.error('Failed to update task status:', error);
    }
  };

  const formatDueDate = (date: Date | undefined) => {
    if (!date) return '';
    const d = new Date(date);
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const tomorrow = new Date(today);
    tomorrow.setDate(tomorrow.getDate() + 1);
    const taskDate = new Date(d);
    taskDate.setHours(0, 0, 0, 0);
    
    if (taskDate.getTime() === today.getTime()) {
      return '今日';
    } else if (taskDate.getTime() === tomorrow.getTime()) {
      return '明日';
    } else if (taskDate < today) {
      return `期限切れ`;
    } else {
      return d.toLocaleDateString('ja-JP', { month: 'short', day: 'numeric' });
    }
  };

  const getDueDateColor = (date: Date | undefined) => {
    if (!date) return '';
    const d = new Date(date);
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    
    if (d < today) {
      return 'text-red-600 font-semibold';
    } else if (d.getTime() === today.getTime()) {
      return 'text-orange-600 font-semibold';
    } else {
      return 'text-gray-600';
    }
  };

  const hasChildren = children.length > 0;
  const progress = parentTask.progress || 0;

  return (
    <div className="space-y-2">
      {/* 進捗率表示 */}
      {hasChildren && (
        <ProgressBar progress={progress} className="mb-2" />
      )}

      {/* 子タスクセクション */}
      <div className="flex items-center gap-2">
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-800"
        >
          <span className={`transform transition-transform ${isExpanded ? 'rotate-90' : ''}`}>
            ▶
          </span>
          子タスク ({children.length})
        </button>
        
        <button
          onClick={() => setShowAddDialog(true)}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          + 追加
        </button>
      </div>

      {/* 子タスクリスト */}
      {isExpanded && (
        <div className="ml-4 space-y-2">
          {isLoading ? (
            <div className="text-sm text-gray-500">読み込み中...</div>
          ) : children.length === 0 ? (
            <div className="text-sm text-gray-500">子タスクがありません</div>
          ) : (
            children.map(child => (
              <div 
                key={child.id} 
                className="flex items-start gap-2 p-2 rounded-lg hover:bg-gray-50 group"
              >
                <input
                  type="checkbox"
                  checked={child.status === 'done'}
                  onChange={(e) => handleStatusChange(child.id, e.target.checked)}
                  className="mt-1"
                />
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className={`text-sm ${child.status === 'done' ? 'line-through text-gray-500' : ''}`}>
                      {child.title}
                    </span>
                    {child.dueDate && (
                      <span className={`text-xs ${getDueDateColor(child.dueDate)}`}>
                        {formatDueDate(child.dueDate)}
                      </span>
                    )}
                    {child.notificationSettings && child.notificationSettings.notificationType !== 'none' && (
                      <span className="text-xs text-blue-500" title="通知設定あり">
                        🔔
                      </span>
                    )}
                  </div>
                  {child.description && (
                    <p className="text-xs text-gray-600 mt-1">{child.description}</p>
                  )}
                </div>
                <div className="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                  <button
                    onClick={() => handleEditSubtask(child)}
                    className="text-xs text-blue-600 hover:text-blue-800"
                  >
                    編集
                  </button>
                  <button
                    onClick={() => handleDelete(child.id)}
                    className="text-xs text-red-600 hover:text-red-800"
                  >
                    削除
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      )}

      {/* 子タスク追加/編集ダイアログ */}
      {showAddDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full max-h-[90vh] overflow-y-auto">
            <h3 className="text-lg font-semibold mb-4">
              {editingSubtask ? '子タスクを編集' : '子タスクを追加'}
            </h3>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  タイトル <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  value={newSubtask.title}
                  onChange={(e) => setNewSubtask({ ...newSubtask, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="タスクのタイトル"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  説明
                </label>
                <textarea
                  value={newSubtask.description}
                  onChange={(e) => setNewSubtask({ ...newSubtask, description: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  rows={3}
                  placeholder="タスクの詳細説明"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  期日
                </label>
                <input
                  type="date"
                  value={newSubtask.dueDate}
                  onChange={(e) => setNewSubtask({ ...newSubtask, dueDate: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>


              {/* 通知設定 */}
              {newSubtask.dueDate && (
                <div className="border-t pt-4">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    通知設定
                  </label>
                  
                  <div className="space-y-3">
                    <div>
                      <label className="block text-sm text-gray-600 mb-1">
                        通知タイプ
                      </label>
                      <select
                        value={newSubtask.notificationType}
                        onChange={(e) => setNewSubtask({ 
                          ...newSubtask, 
                          notificationType: e.target.value as 'none' | 'due_date_based' | 'recurring' 
                        })}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      >
                        <option value="none">通知なし</option>
                        <option value="due_date_based">期日通知</option>
                        <option value="recurring">定期通知</option>
                      </select>
                    </div>

                    {newSubtask.notificationType !== 'none' && (
                      <>
                        <div>
                          <label className="block text-sm text-gray-600 mb-1">
                            何日前に通知
                          </label>
                          <input
                            type="number"
                            min="0"
                            max="30"
                            value={newSubtask.daysBefore}
                            onChange={(e) => setNewSubtask({ 
                              ...newSubtask, 
                              daysBefore: parseInt(e.target.value) || 1 
                            })}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                          />
                        </div>

                        <div>
                          <label className="block text-sm text-gray-600 mb-1">
                            通知時刻
                          </label>
                          <input
                            type="time"
                            value={newSubtask.notificationTime}
                            onChange={(e) => setNewSubtask({ 
                              ...newSubtask, 
                              notificationTime: e.target.value 
                            })}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                          />
                        </div>
                      </>
                    )}
                  </div>
                </div>
              )}
            </div>

            <div className="flex gap-2 mt-6">
              <button
                onClick={handleSaveSubtask}
                className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors"
              >
                {editingSubtask ? '更新' : '追加'}
              </button>
              <button
                onClick={() => {
                  setShowAddDialog(false);
                  setEditingSubtask(null);
                  setNewSubtask({
                    title: '',
                    description: '',
                    dueDate: '',
                                notificationType: 'none',
                    daysBefore: 1,
                    notificationTime: '09:00',
                  });
                }}
                className="flex-1 bg-gray-200 text-gray-800 py-2 px-4 rounded-md hover:bg-gray-300 transition-colors"
              >
                キャンセル
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
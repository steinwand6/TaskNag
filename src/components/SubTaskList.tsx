import React, { useState, useEffect } from 'react';
import { Task } from '../types/Task';
import { TaskService } from '../services/taskService';
import { ProgressBar } from './ProgressBar';
import { ChevronDownIcon, ChevronRightIcon, PlusIcon } from '@heroicons/react/24/outline';

interface SubTaskListProps {
  parentTask: Task;
  onTaskUpdate?: (task: Task) => void;
}

export const SubTaskList: React.FC<SubTaskListProps> = ({ parentTask, onTaskUpdate }) => {
  const [children, setChildren] = useState<Task[]>([]);
  const [isExpanded, setIsExpanded] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  // ローカルでの進捗率計算
  const calculateProgress = (childTasks: Task[]) => {
    if (childTasks.length === 0) return 0;
    const completedCount = childTasks.filter(child => child.status === 'done').length;
    return Math.round((completedCount / childTasks.length) * 100);
  };

  const loadChildren = async () => {
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
  };

  useEffect(() => {
    if (isExpanded) {
      loadChildren();
    }
  }, [isExpanded, parentTask.id]);

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

  const handleAddSubtask = async () => {
    const title = prompt('子タスクのタイトルを入力してください:');
    if (!title || !title.trim()) return;

    try {
      const newSubtask = await TaskService.createTask({
        title: title.trim(),
        description: '',
        status: 'todo',
        priority: 'medium',
        parentId: parentTask.id,
        dueDate: undefined,
      });

      // 子タスクリストを更新
      const updatedChildren = [...children, newSubtask];
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
      console.error('Failed to create subtask:', error);
      alert('子タスクの作成に失敗しました');
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

  const hasChildren = children.length > 0;
  const progress = parentTask.progress || 0;

  return (
    <div className="space-y-2">
      {/* 進捗率表示 */}
      {hasChildren && (
        <ProgressBar progress={progress} className="mb-2" />
      )}

      {/* 子タスク展開ボタン */}
      <div className="flex items-center gap-2">
        <button
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            setIsExpanded(!isExpanded);
          }}
          className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-800"
          disabled={isLoading}
        >
          {isExpanded ? (
            <ChevronDownIcon className="w-4 h-4" />
          ) : (
            <ChevronRightIcon className="w-4 h-4" />
          )}
          子タスク {hasChildren && `(${children.length})`}
        </button>
        
        {isExpanded && (
          <button
            className="flex items-center gap-1 text-sm text-blue-600 hover:text-blue-800"
            onClick={(e) => {
              e.stopPropagation();
              e.preventDefault();
              handleAddSubtask();
            }}
          >
            <PlusIcon className="w-4 h-4" />
            追加
          </button>
        )}
      </div>

      {/* 子タスクリスト */}
      {isExpanded && (
        <div className="ml-6 space-y-2 border-l-2 border-gray-200 pl-4">
          {isLoading ? (
            <div className="text-sm text-gray-500">読み込み中...</div>
          ) : children.length === 0 ? (
            <div className="text-sm text-gray-500">子タスクはありません</div>
          ) : (
            children.map(child => (
              <div key={child.id} className="bg-gray-50 rounded p-3 flex items-center justify-between">
                <div className="flex items-center gap-3 flex-1">
                  <input
                    type="checkbox"
                    checked={child.status === 'done'}
                    onChange={(e) => handleStatusChange(child.id, e.target.checked)}
                    onClick={(e) => e.stopPropagation()}
                    className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
                  />
                  <div className="flex-1">
                    <h4 className={`font-medium text-sm ${
                      child.status === 'done' ? 'line-through text-gray-500' : 'text-gray-900'
                    }`}>
                      {child.title}
                    </h4>
                    {child.description && (
                      <p className="text-xs text-gray-600 mt-1">{child.description}</p>
                    )}
                  </div>
                </div>
                
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDelete(child.id);
                  }}
                  className="text-red-500 hover:text-red-700 text-sm ml-2"
                  title="削除"
                >
                  ×
                </button>
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
};
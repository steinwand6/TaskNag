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

  const handleProgressUpdate = async (taskId: string, progress: number) => {
    try {
      const updatedTask = await TaskService.updateProgress(taskId, progress);
      
      // 子タスクリストを更新
      setChildren(prev => 
        prev.map(child => 
          child.id === taskId ? updatedTask : child
        )
      );
      
      // 親タスクの進捗率を再計算
      if (parentTask.id) {
        await TaskService.calculateAndUpdateProgress(parentTask.id);
        // 親タスクの更新を通知
        if (onTaskUpdate) {
          const updatedParent = await TaskService.getTaskById(parentTask.id);
          onTaskUpdate(updatedParent);
        }
      }
    } catch (error) {
      console.error('Failed to update progress:', error);
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
      setChildren(prev => [...prev, newSubtask]);
      
      // 親タスクの進捗率を再計算
      if (parentTask.id) {
        await TaskService.calculateAndUpdateProgress(parentTask.id);
        if (onTaskUpdate) {
          const updatedParent = await TaskService.getTaskById(parentTask.id);
          onTaskUpdate(updatedParent);
        }
      }
    } catch (error) {
      console.error('Failed to create subtask:', error);
      alert('子タスクの作成に失敗しました');
    }
  };

  const handleStatusChange = async (taskId: string, newStatus: string) => {
    try {
      const updatedTask = await TaskService.moveTask(taskId, newStatus as any);
      
      // ステータス変更時に進捗率も自動更新
      if (newStatus === 'done') {
        await handleProgressUpdate(taskId, 100);
      } else if (parentTask.status === 'done' && newStatus !== 'done') {
        await handleProgressUpdate(taskId, 0);
      }
      
      // 子タスクリストを更新
      setChildren(prev => 
        prev.map(child => 
          child.id === taskId ? updatedTask : child
        )
      );
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
              <div key={child.id} className="bg-gray-50 rounded p-3 space-y-2">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <h4 className="font-medium text-sm">{child.title}</h4>
                    {child.description && (
                      <p className="text-xs text-gray-600 mt-1">{child.description}</p>
                    )}
                  </div>
                  
                  <select
                    value={child.status}
                    onChange={(e) => handleStatusChange(child.id, e.target.value)}
                    onClick={(e) => e.stopPropagation()}
                    className="text-xs border rounded px-2 py-1"
                  >
                    <option value="inbox">Inbox</option>
                    <option value="todo">Todo</option>
                    <option value="in_progress">進行中</option>
                    <option value="done">完了</option>
                  </select>
                </div>
                
                {/* 子タスクの進捗率コントロール */}
                <div className="flex items-center gap-2">
                  <span className="text-xs text-gray-500">進捗:</span>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={child.progress || 0}
                    onChange={(e) => handleProgressUpdate(child.id, parseInt(e.target.value))}
                    onClick={(e) => e.stopPropagation()}
                    className="flex-1"
                  />
                  <span className="text-xs text-gray-600 min-w-[3rem]">
                    {child.progress || 0}%
                  </span>
                </div>
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
};
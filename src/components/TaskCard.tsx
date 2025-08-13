import React from 'react';
import { Task, TaskStatus } from '../types/Task';
import { useTaskStore } from '../stores/taskStore';
import { EditTaskModal } from './EditTaskModal';
import { SubTaskList } from './SubTaskList';

interface TaskCardProps {
  task: Task;
  onDragStart?: (taskId: string) => void;
  onDragEnd?: () => void;
}

export const TaskCard: React.FC<TaskCardProps> = ({ 
  task, 
  onDragStart, 
  onDragEnd 
}) => {
  const { deleteTask, moveTask } = useTaskStore();
  const [editTask, setEditTask] = React.useState<Task | null>(null);
  const [isDragging, setIsDragging] = React.useState(false);
  
  const getPriorityColor = (priority: Task['priority']) => {
    switch (priority) {
      case 'required': return 'border-l-red-600';
      case 'high': return 'border-l-red-500';
      case 'medium': return 'border-l-yellow-500';
      case 'low': return 'border-l-green-500';
      default: return 'border-l-gray-300';
    }
  };
  
  const formatDate = (date?: Date) => {
    if (!date) return '';
    return new Intl.DateTimeFormat('ja-JP', {
      month: 'short',
      day: 'numeric',
    }).format(date);
  };

  const handleDoubleClick = () => {
    setEditTask(task);
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    if (e.button !== 0) return; // 左クリックのみ
    
    const startPos = { x: e.clientX, y: e.clientY };
    
    let dragStarted = false;
    
    const handleMouseMove = (e: MouseEvent) => {
      const deltaX = Math.abs(e.clientX - startPos.x);
      const deltaY = Math.abs(e.clientY - startPos.y);
      
      if ((deltaX > 3 || deltaY > 3) && !dragStarted) {
        dragStarted = true;
        setIsDragging(true);
        onDragStart?.(task.id);
      }
    };
    
    const handleMouseUp = (e: MouseEvent) => {
      if (dragStarted) {
        // ドロップ処理：マウスアップ位置の要素を取得
        const elementUnderMouse = document.elementFromPoint(e.clientX, e.clientY);
        
        // カラム要素を探す
        let columnElement = elementUnderMouse;
        let status = null;
        
        while (columnElement && columnElement !== document.body) {
          if (columnElement.hasAttribute && columnElement.hasAttribute('data-status')) {
            status = columnElement.getAttribute('data-status') as TaskStatus;
            break;
          }
          columnElement = columnElement.parentElement;
        }
        
        if (status) {
          moveTask(task.id, status);
        }
        
        setIsDragging(false);
        onDragEnd?.();
      }
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  };
  
  return (
    <>
      <div 
        className={`bg-white p-3 rounded-lg border-l-4 ${getPriorityColor(task.priority)} shadow-sm hover:shadow-md transition-shadow cursor-move select-none ${
          isDragging ? 'opacity-50 scale-105' : ''
        }`}
        onDoubleClick={handleDoubleClick}
        onMouseDown={handleMouseDown}
      >
        <div className="flex justify-between items-start mb-2">
          <h4 className="font-medium text-gray-900 text-sm leading-tight">
            {task.title}
          </h4>
          <button
            onClick={(e) => {
              e.stopPropagation();
              deleteTask(task.id);
            }}
            className={`text-gray-400 hover:text-red-500 text-xs ml-2 ${
              isDragging ? 'pointer-events-none opacity-50' : ''
            }`}
          >
            ×
          </button>
        </div>
        
        {task.description && (
          <p className="text-gray-600 text-xs mb-2 line-clamp-2">
            {task.description}
          </p>
        )}
        
        {/* 子タスクリスト */}
        <SubTaskList 
          parentTask={task} 
          onTaskUpdate={(updatedTask) => {
            // タスクの更新をストアに反映
            // この実装は簡易的なもので、実際にはもっと適切な更新処理が必要
            console.log('Task updated:', updatedTask);
          }}
        />
        
        <div className="flex justify-between items-center text-xs text-gray-500">
          <span className="capitalize">{task.priority}</span>
          {task.dueDate && (
            <span className="text-orange-600">
              📅 {formatDate(task.dueDate)}
            </span>
          )}
        </div>
      </div>
      
      <EditTaskModal
        isOpen={editTask !== null}
        onClose={() => setEditTask(null)}
        task={editTask}
      />
    </>
  );
};
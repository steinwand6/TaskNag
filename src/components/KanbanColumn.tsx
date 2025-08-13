import React from 'react';
import { Task, TaskStatus } from '../types/Task';
import { TaskCard } from './TaskCard';

interface StatusData {
  title: string;
  subtitle: string;
  color: string;
  count: number;
  tasks: Task[];
}

interface KanbanColumnProps {
  status: TaskStatus;
  statusData: StatusData;
  isLoading: boolean;
  isDragOver: boolean;
  onMouseEnter: (status: TaskStatus) => void;
  onMouseLeave: () => void;
  onClick: (status: TaskStatus) => void;
  onDragStart: (taskId: string) => void;
  onDragEnd: () => void;
  onNewTask: (status: TaskStatus) => void;
}

export const KanbanColumn: React.FC<KanbanColumnProps> = ({
  status,
  statusData,
  isLoading,
  isDragOver,
  onMouseEnter,
  onMouseLeave,
  onClick,
  onDragStart,
  onDragEnd,
  onNewTask,
}) => {
  return (
    <div 
      data-status={status}
      className={`bg-white rounded-lg shadow-sm border border-gray-200 transition-colors ${
        isDragOver ? 'ring-2 ring-blue-400 border-blue-400 bg-blue-50' : ''
      }`}
      onMouseEnter={() => onMouseEnter(status)}
      onMouseLeave={onMouseLeave}
      onClick={() => onClick(status)}
    >
      <div className={`${statusData.color} text-white p-4 rounded-t-lg`}>
        <h2 className="text-lg font-semibold">{statusData.title}</h2>
        <p className="text-sm opacity-90">{statusData.subtitle} ({statusData.count})</p>
      </div>
      
      <div className="p-4 space-y-3">
        <div className="min-h-[200px]">
          {statusData.tasks.map((task) => (
            <TaskCard 
              key={task.id} 
              task={task} 
              onDragStart={onDragStart}
              onDragEnd={onDragEnd}
            />
          ))}
          
          {statusData.tasks.length === 0 && !isLoading && (
            <div className="text-center text-gray-400 py-8">
              <p className="text-sm">タスクがありません</p>
              <button 
                onClick={() => onNewTask(status)} 
                className="text-blue-500 text-xs hover:underline mt-2"
              >
                + 新規追加
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
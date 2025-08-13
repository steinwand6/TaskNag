import React from 'react';

interface HeaderProps {
  isLoading: boolean;
  onNewTask: () => void;
  onRefresh: () => void;
}

export const Header: React.FC<HeaderProps> = ({ isLoading, onNewTask, onRefresh }) => {
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
          </div>
        </div>
      </div>
    </header>
  );
};
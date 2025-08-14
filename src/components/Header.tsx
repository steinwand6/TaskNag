import React from 'react';

interface HeaderProps {
  isLoading: boolean;
  onNewTask: () => void;
  showDone: boolean;
  onToggleDone: () => void;
  onManageTags?: () => void;
  onToggleFilters?: () => void;
  showFilters?: boolean;
  hasActiveFilters?: boolean;
  onOpenAgentChat?: () => void;
  onOpenSmartCreator?: () => void;
}

export const Header: React.FC<HeaderProps> = ({ 
  isLoading, 
  onNewTask, 
  showDone, 
  onToggleDone, 
  onManageTags, 
  onToggleFilters, 
  showFilters = false, 
  hasActiveFilters = false,
  onOpenAgentChat,
  onOpenSmartCreator
}) => {
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
            {onToggleFilters && (
              <button 
                onClick={onToggleFilters}
                className={`px-3 py-2 text-sm rounded-md transition-colors relative ${
                  showFilters 
                    ? 'bg-blue-100 text-blue-800 border border-blue-300' 
                    : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
                }`}
                disabled={isLoading}
              >
                🔍 フィルタ
                {hasActiveFilters && (
                  <span className="absolute -top-1 -right-1 w-2 h-2 bg-red-500 rounded-full"></span>
                )}
              </button>
            )}
            {onManageTags && (
              <button 
                onClick={onManageTags}
                className="px-3 py-2 text-sm rounded-md bg-purple-100 text-purple-800 border border-purple-300 hover:bg-purple-200 transition-colors"
                disabled={isLoading}
              >
                🏷️ タグ管理
              </button>
            )}
            <button 
              onClick={onToggleDone}
              className={`px-3 py-2 text-sm rounded-md transition-colors ${
                showDone 
                  ? 'bg-green-100 text-green-800 border border-green-300' 
                  : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
              }`}
              disabled={isLoading}
            >
              {showDone ? '✅ DONE表示中' : '✅ DONE'}
            </button>
            {onOpenSmartCreator && (
              <button 
                onClick={onOpenSmartCreator}
                className="px-3 py-2 text-sm rounded-md bg-purple-100 text-purple-800 border border-purple-300 hover:bg-purple-200 transition-colors"
                disabled={isLoading}
                title="AIでタスクを作成"
              >
                🤖 AI作成
              </button>
            )}
            {onOpenAgentChat && (
              <button 
                onClick={onOpenAgentChat}
                className="px-3 py-2 text-sm rounded-md bg-blue-100 text-blue-800 border border-blue-300 hover:bg-blue-200 transition-colors"
                disabled={isLoading}
                title="AIアシスタントとチャット"
              >
                💬 AI相談
              </button>
            )}
            <button 
              onClick={onNewTask}
              className="btn-primary"
              disabled={isLoading}
            >
              + 新規タスク
            </button>
          </div>
        </div>
      </div>
    </header>
  );
};
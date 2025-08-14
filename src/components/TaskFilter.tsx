import React from 'react';
import { useTaskStore } from '../stores/taskStore';
import { TagDisplay } from './TagDisplay';
import { MagnifyingGlassIcon, XMarkIcon, FunnelIcon, EyeIcon, EyeSlashIcon } from '@heroicons/react/24/outline';

interface TaskFilterProps {
  className?: string;
}

export const TaskFilter: React.FC<TaskFilterProps> = ({ className = '' }) => {
  const {
    tags,
    selectedTags,
    searchQuery,
    showCompletedTasks,
    setSearchQuery,
    toggleTag,
    clearTagFilter,
    setShowCompletedTasks,
  } = useTaskStore();

  const selectedTagObjects = tags.filter(tag => selectedTags.includes(tag.id));
  const activeFiltersCount = selectedTags.length + (searchQuery ? 1 : 0) + (showCompletedTasks ? 0 : 1);

  return (
    <div className={`bg-white rounded-lg shadow-sm border p-4 space-y-4 ${className}`}>
      {/* ヘッダー */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <FunnelIcon className="w-5 h-5 text-gray-500" />
          <h3 className="font-medium text-gray-900">フィルタ</h3>
          {activeFiltersCount > 0 && (
            <span className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full">
              {activeFiltersCount}
            </span>
          )}
        </div>
        {(selectedTags.length > 0 || searchQuery) && (
          <button
            onClick={() => {
              clearTagFilter();
              setSearchQuery('');
            }}
            className="text-sm text-gray-500 hover:text-gray-700 flex items-center gap-1"
          >
            <XMarkIcon className="w-4 h-4" />
            クリア
          </button>
        )}
      </div>

      {/* 検索バー */}
      <div className="relative">
        <MagnifyingGlassIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
        <input
          type="text"
          placeholder="タスクを検索..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
        />
      </div>

      {/* 完了タスク表示切り替え */}
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium text-gray-700">完了タスクを表示</span>
        <button
          onClick={() => setShowCompletedTasks(!showCompletedTasks)}
          className={`flex items-center gap-1 px-3 py-1 rounded-md transition-colors ${
            showCompletedTasks
              ? 'bg-green-100 text-green-700 hover:bg-green-200'
              : 'bg-gray-100 text-gray-500 hover:bg-gray-200'
          }`}
        >
          {showCompletedTasks ? (
            <><EyeIcon className="w-4 h-4" />表示中</>
          ) : (
            <><EyeSlashIcon className="w-4 h-4" />非表示</>
          )}
        </button>
      </div>

      {/* タグフィルタ */}
      <div>
        <h4 className="text-sm font-medium text-gray-700 mb-2">タグで絞り込み</h4>
        {tags.length === 0 ? (
          <p className="text-sm text-gray-500">タグがありません</p>
        ) : (
          <div className="space-y-2">
            {/* 利用可能タグ */}
            <div className="flex flex-wrap gap-1">
              {tags.map((tag) => (
                <button
                  key={tag.id}
                  onClick={() => toggleTag(tag.id)}
                  className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border transition-all duration-200 ${
                    selectedTags.includes(tag.id)
                      ? 'border-gray-400 shadow-md transform scale-105'
                      : 'border-gray-200 hover:border-gray-300 hover:shadow-sm'
                  }`}
                  style={{
                    backgroundColor: selectedTags.includes(tag.id) 
                      ? tag.color + '20' 
                      : tag.color + '10',
                    color: tag.color,
                  }}
                  title={selectedTags.includes(tag.id) ? 'クリックで選択解除' : 'クリックで選択'}
                >
                  <span>{tag.name}</span>
                  {selectedTags.includes(tag.id) && (
                    <XMarkIcon className="w-3 h-3 ml-1" />
                  )}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* 選択中のタグ */}
      {selectedTags.length > 0 && (
        <div>
          <div className="flex items-center justify-between mb-2">
            <h4 className="text-sm font-medium text-gray-700">選択中のタグ</h4>
            <button
              onClick={clearTagFilter}
              className="text-xs text-gray-500 hover:text-gray-700"
            >
              すべて解除
            </button>
          </div>
          <TagDisplay
            tags={selectedTagObjects}
            maxDisplay={10}
            size="sm"
            onClick={(tag) => toggleTag(tag.id)}
            showRemoveButton={false}
          />
        </div>
      )}

      {/* フィルタ結果統計 */}
      <div className="pt-2 border-t border-gray-200">
        <p className="text-xs text-gray-500">
          {activeFiltersCount > 0 ? (
            `${activeFiltersCount}個のフィルタが適用されています`
          ) : (
            'フィルタが適用されていません'
          )}
        </p>
      </div>
    </div>
  );
};
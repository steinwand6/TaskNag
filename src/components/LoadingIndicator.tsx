import React from 'react';

export const LoadingIndicator: React.FC = () => {
  return (
    <div className="mb-4 text-center">
      <div className="inline-flex items-center space-x-2">
        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-500"></div>
        <span className="text-gray-600">読み込み中...</span>
      </div>
    </div>
  );
};
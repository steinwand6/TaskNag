import React from 'react';
import { LogService } from '../services/logService';

interface ErrorMessageProps {
  error: Error;
}

export const ErrorMessage: React.FC<ErrorMessageProps> = ({ error }) => {
  // エラー内容をログファイルに出力
  React.useEffect(() => {
    console.error('ErrorMessage component received error:', error);
    console.error('Error stack:', error.stack);
    
    LogService.error('TaskNag Application Error', {
      message: error.message,
      stack: error.stack,
      name: error.name,
    });
  }, [error]);
  
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded max-w-md">
        <p className="font-bold">エラーが発生しました</p>
        <p className="text-sm mt-2">エラー: {error.name}</p>
        <p className="text-sm">メッセージ: {error.message}</p>
        <p className="text-xs text-red-600 mt-2">エラー詳細はログファイルに記録されました</p>
      </div>
    </div>
  );
};
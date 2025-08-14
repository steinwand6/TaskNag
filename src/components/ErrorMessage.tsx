import React from 'react';
import { LogService } from '../services/logService';

interface ErrorMessageProps {
  error: Error;
}

export const ErrorMessage: React.FC<ErrorMessageProps> = ({ error }) => {
  // エラー内容をログファイルに出力
  React.useEffect(() => {
    LogService.error('TaskNag Application Error', {
      message: error.message,
      stack: error.stack,
      name: error.name,
    });
  }, [error]);
  
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
        <p className="font-bold">エラーが発生しました</p>
        <p>詳細はブラウザの開発者ツール（F12）のConsoleタブをご確認ください</p>
      </div>
    </div>
  );
};
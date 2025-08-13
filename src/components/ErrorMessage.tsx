import React from 'react';

interface ErrorMessageProps {
  error: Error;
}

export const ErrorMessage: React.FC<ErrorMessageProps> = ({ error }) => {
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
        <p className="font-bold">エラーが発生しました</p>
        <p>{error.message}</p>
      </div>
    </div>
  );
};
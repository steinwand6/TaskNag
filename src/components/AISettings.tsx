import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { PersonalitySelector } from './PersonalitySelector';
import { ModelSelector } from './ModelSelector';
import { LogService } from '../services/logService';

interface AISettingsProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AISettings: React.FC<AISettingsProps> = ({ isOpen, onClose }) => {
  const [isTestingConnection, setIsTestingConnection] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<'connected' | 'disconnected' | 'testing'>('disconnected');
  const [error, setError] = useState<string | null>(null);

  // Ollama接続テスト
  const testConnection = async () => {
    try {
      setIsTestingConnection(true);
      setConnectionStatus('testing');
      setError(null);
      
      const isConnected = await invoke<boolean>('test_ollama_connection');
      
      if (isConnected) {
        setConnectionStatus('connected');
        LogService.info('AISettings', 'Ollama接続成功');
      } else {
        setConnectionStatus('disconnected');
        setError('Ollamaサービスに接続できません');
      }
    } catch (err) {
      setConnectionStatus('disconnected');
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      LogService.error('AISettings', `Ollama接続エラー: ${errorMessage}`);
    } finally {
      setIsTestingConnection(false);
    }
  };


  // モーダルが開いたら接続テスト
  useEffect(() => {
    if (isOpen) {
      testConnection();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <>
      {/* オーバーレイ */}
      <div 
        className="fixed inset-0 bg-black bg-opacity-50 z-40"
        onClick={onClose}
      />
      
      {/* モーダル */}
      <div className="fixed inset-0 flex items-center justify-center z-50 p-4">
        <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
          {/* ヘッダー */}
          <div className="sticky top-0 bg-white border-b px-6 py-4">
            <div className="flex items-center justify-between">
              <h2 className="text-2xl font-bold text-gray-800">AI設定</h2>
              <button
                onClick={onClose}
                className="p-2 hover:bg-gray-100 rounded-full transition-colors"
                aria-label="閉じる"
              >
                <svg
                  className="w-6 h-6 text-gray-500"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
          </div>

          {/* コンテンツ */}
          <div className="p-6 space-y-6">
            {/* Ollama接続状態 */}
            <div className="bg-gray-50 rounded-lg p-4">
              <h3 className="text-lg font-semibold text-gray-800 mb-3">Ollama接続状態</h3>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-3">
                  <div className={`w-3 h-3 rounded-full ${
                    connectionStatus === 'connected' ? 'bg-green-500' :
                    connectionStatus === 'testing' ? 'bg-yellow-500 animate-pulse' :
                    'bg-red-500'
                  }`} />
                  <span className="text-sm font-medium text-gray-700">
                    {connectionStatus === 'connected' ? '接続済み' :
                     connectionStatus === 'testing' ? '接続テスト中...' :
                     '未接続'}
                  </span>
                </div>
                
                <button
                  onClick={testConnection}
                  disabled={isTestingConnection}
                  className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
                >
                  {isTestingConnection ? '接続テスト中...' : '接続テスト'}
                </button>
              </div>

              {error && (
                <div className={`mt-3 p-3 rounded-md ${
                  error.startsWith('SUCCESS:')
                    ? 'bg-green-50 border border-green-200'
                    : 'bg-red-50 border border-red-200'
                }`}>
                  <p className={`text-sm ${
                    error.startsWith('SUCCESS:')
                      ? 'text-green-600'
                      : 'text-red-600'
                  }`}>
                    {error.startsWith('SUCCESS:') ? error.substring(8) : error}
                  </p>
                  {!error.startsWith('SUCCESS:') && (
                    <p className="text-xs text-red-500 mt-1">
                      Ollamaが起動していることを確認してください
                    </p>
                  )}
                </div>
              )}


            </div>

            {/* AIモデル設定 */}
            {connectionStatus === 'connected' && (
              <ModelSelector 
                onModelChange={(model) => {
                  LogService.info('AISettings', `モデル変更通知: ${model}`);
                }}
              />
            )}

            {/* AI性格設定 */}
            <PersonalitySelector 
              onPersonalityChange={(personality) => {
                if (personality) {
                  LogService.info('AISettings', `性格変更: ${personality.name}`);
                }
              }}
            />

            {/* 使い方ガイド */}
            <div className="bg-blue-50 rounded-lg p-4">
              <h3 className="text-lg font-semibold text-blue-800 mb-3">AIアシスタントの使い方</h3>
              <div className="space-y-2 text-sm text-blue-700">
                <p>🤖 タスクの説明を入力すると、AIが分析・改善提案をします</p>
                <p>📝 自然言語でタスクを作成できます（例：「明日までにレポートを書く」）</p>
                <p>💬 AIとチャットして、タスク管理のアドバイスを受けられます</p>
                <p>🎭 性格を変更すると、AIの応答スタイルが変わります</p>
              </div>
            </div>

            {/* 注意事項 */}
            <div className="bg-yellow-50 rounded-lg p-4">
              <h3 className="text-lg font-semibold text-yellow-800 mb-3">注意事項</h3>
              <div className="space-y-2 text-sm text-yellow-700">
                <p>⚠️ AI機能を使用するには、Ollamaがローカルで実行されている必要があります</p>
                <p>💾 すべてのデータはローカルに保存され、外部に送信されません</p>
                <p>🔒 プライバシーを保護しながら、高度なAI機能を利用できます</p>
              </div>
            </div>
          </div>

          {/* フッター */}
          <div className="sticky bottom-0 bg-gray-50 border-t px-6 py-4">
            <div className="flex justify-end">
              <button
                onClick={onClose}
                className="px-6 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 transition-colors"
              >
                閉じる
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};
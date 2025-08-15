import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ModelInfo, ModelPreference } from '../types/AI';
import { LogService } from '../services/logService';

interface ModelSelectorProps {
  onModelChange?: (model: string) => void;
}

export const ModelSelector: React.FC<ModelSelectorProps> = ({ onModelChange }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [models, setModels] = useState<string[]>([]);
  const [detailedModels, setDetailedModels] = useState<ModelInfo[]>([]);
  const [currentModel, setCurrentModel] = useState<string>('');
  const [selectedModel, setSelectedModel] = useState<string>('');
  const [isChangingModel, setIsChangingModel] = useState(false);
  const [modelPreferences, setModelPreferences] = useState<Record<string, ModelPreference>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // モデル情報を取得
  const loadModelInfo = async () => {
    try {
      setIsLoading(true);
      setError(null);

      const [modelList, detailedModelList, current, preferences] = await Promise.all([
        invoke<string[]>('list_ollama_models'),
        invoke<ModelInfo[]>('list_ollama_models_detailed'),
        invoke<string>('get_current_model'),
        invoke<Record<string, ModelPreference>>('get_model_preferences_for_available_models')
      ]);

      setModels(modelList);
      setDetailedModels(detailedModelList);
      setCurrentModel(current);
      setSelectedModel(current);
      setModelPreferences(preferences);
      
      LogService.info('ModelSelector', `モデル情報取得成功: ${modelList.length}個のモデル, 使用中: ${current}`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      LogService.error('ModelSelector', `モデル情報取得エラー: ${errorMessage}`);
    } finally {
      setIsLoading(false);
    }
  };

  // モデル変更
  const changeModel = async (newModel: string) => {
    if (newModel === currentModel) return;

    try {
      setIsChangingModel(true);
      setError(null);

      await invoke('set_current_model', { model: newModel });
      setSelectedModel(newModel);
      
      LogService.info('ModelSelector', `モデル変更: ${currentModel} → ${newModel} (次回起動時に反映)`);
      onModelChange?.(newModel);

      // 成功メッセージを表示
      setError(`SUCCESS:モデルを「${newModel}」に設定しました。アプリケーション再起動後に反映されます。`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`モデル変更に失敗しました: ${errorMessage}`);
      LogService.error('ModelSelector', `モデル変更エラー: ${errorMessage}`);
      setSelectedModel(currentModel);
    } finally {
      setIsChangingModel(false);
    }
  };

  // パフォーマンス階層の表示文字列
  const getPerformanceTierDisplay = (tier: string) => {
    switch (tier) {
      case 'Fast': return '⚡ 高速';
      case 'Balanced': return '⚖️ バランス';
      case 'Quality': return '🎯 高品質';
      default: return '';
    }
  };

  // コンポーネントマウント時にモデル情報を取得
  useEffect(() => {
    loadModelInfo();
  }, []);

  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <h3 className="text-lg font-semibold text-gray-800 mb-1">AIモデル設定</h3>
          {currentModel && (
            <div className="flex items-center space-x-2">
              <span className="text-sm text-gray-600">使用中:</span>
              <span className="px-3 py-1 bg-blue-600 text-white rounded-full text-sm font-medium">
                {currentModel}
              </span>
              {modelPreferences[currentModel] && (
                <span className="text-xs text-gray-500">
                  {getPerformanceTierDisplay(modelPreferences[currentModel].performance_tier)}
                </span>
              )}
            </div>
          )}
        </div>
        
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="p-2 hover:bg-gray-200 rounded-full transition-colors"
          aria-label={isExpanded ? 'モデル設定を閉じる' : 'モデル設定を開く'}
        >
          <svg
            className={`w-5 h-5 text-gray-500 transform transition-transform ${isExpanded ? 'rotate-180' : ''}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </button>
      </div>

      {isExpanded && (
        <div className="mt-4 space-y-4">
          {/* リロードボタン */}
          <div className="flex justify-end">
            <button
              onClick={loadModelInfo}
              disabled={isLoading}
              className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
            >
              {isLoading ? '更新中...' : '情報を更新'}
            </button>
          </div>

          {/* エラーメッセージ */}
          {error && (
            <div className={`p-3 rounded-md ${
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
            </div>
          )}

          {/* モデル選択 */}
          {models.length > 0 && (
            <div className="space-y-3">
              <div className="p-3 bg-green-50 border border-green-200 rounded-md">
                <p className="text-sm font-medium text-green-800 mb-2">利用可能なモデル (クリックで変更):</p>
                <div className="flex flex-wrap gap-2">
                  {models.map((model) => (
                    <button
                      key={model}
                      onClick={() => changeModel(model)}
                      disabled={isChangingModel}
                      className={`px-3 py-2 rounded-md text-xs font-medium transition-colors ${
                        selectedModel === model
                          ? 'bg-blue-600 text-white border-2 border-blue-700'
                          : model === currentModel
                          ? 'bg-green-600 text-white border-2 border-green-700'
                          : 'bg-white text-green-700 border border-green-300 hover:bg-green-100'
                      } ${isChangingModel ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}`}
                      title={
                        selectedModel === model
                          ? '次回起動時に適用されます'
                          : model === currentModel
                          ? '現在使用中'
                          : 'クリックして選択'
                      }
                    >
                      {model}
                      {selectedModel === model && selectedModel !== currentModel && (
                        <span className="ml-1">⏱</span>
                      )}
                      {model === currentModel && (
                        <span className="ml-1">✓</span>
                      )}
                    </button>
                  ))}
                </div>
              </div>

              {/* 詳細モデル情報 */}
              {detailedModels.length > 0 && (
                <div className="p-3 bg-blue-50 border border-blue-200 rounded-md">
                  <p className="text-sm font-medium text-blue-800 mb-3">モデル詳細情報:</p>
                  <div className="space-y-2 max-h-48 overflow-y-auto">
                    {detailedModels.map((model) => {
                      const preference = modelPreferences[model.name];
                      return (
                        <div 
                          key={model.name}
                          className={`p-3 rounded border-l-4 ${
                            model.name === currentModel 
                              ? 'bg-green-100 border-green-500' 
                              : selectedModel === model.name 
                              ? 'bg-blue-100 border-blue-500'
                              : 'bg-white border-gray-300'
                          }`}
                        >
                          <div className="flex items-center justify-between mb-2">
                            <div className="flex items-center space-x-2">
                              <span className="font-medium text-sm">{model.name}</span>
                              {preference && (
                                <span className="text-xs px-2 py-1 bg-gray-100 text-gray-600 rounded">
                                  {getPerformanceTierDisplay(preference.performance_tier)}
                                </span>
                              )}
                            </div>
                            <div className="flex items-center space-x-2 text-xs text-gray-600">
                              {model.name === currentModel && (
                                <span className="px-2 py-1 bg-green-600 text-white rounded-full">使用中</span>
                              )}
                              {selectedModel === model.name && model.name !== currentModel && (
                                <span className="px-2 py-1 bg-blue-600 text-white rounded-full">選択済み</span>
                              )}
                            </div>
                          </div>
                          
                          <div className="text-xs text-gray-500 space-y-1">
                            <div>サイズ: {(model.size / (1024 * 1024 * 1024)).toFixed(2)} GB</div>
                            <div>更新日時: {new Date(model.modified_at).toLocaleString('ja-JP')}</div>
                            {preference && (
                              <>
                                <div className="mt-2 text-gray-600">{preference.description}</div>
                                {preference.recommended_for.length > 0 && (
                                  <div className="mt-1">
                                    <span className="font-medium">推奨用途:</span> {preference.recommended_for.join(', ')}
                                  </div>
                                )}
                              </>
                            )}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}
            </div>
          )}

          {isLoading && (
            <div className="text-center py-4">
              <div className="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
              <p className="text-sm text-gray-600 mt-2">モデル情報を読み込み中...</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
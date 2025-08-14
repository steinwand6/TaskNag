import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTaskStore } from '../stores/taskStore';
import { TaskStatus } from '../types/Task';

interface SmartTaskCreatorProps {
  isOpen: boolean;
  onClose: () => void;
  initialStatus?: TaskStatus;
}

interface TaskAnalysis {
  improved_title: string;
  improved_description: string;
  suggested_tags: string[];
  complexity: 'simple' | 'medium' | 'complex';
  estimated_hours: number;
  subtasks: Array<{
    title: string;
    description: string;
    order: number;
  }>;
  priority_reasoning: string;
}

interface NaturalLanguageTask {
  title: string;
  description: string;
  suggested_status: TaskStatus;
  due_date_suggestion?: string;
  tags: string[];
  notification_needed: boolean;
}

export const SmartTaskCreator: React.FC<SmartTaskCreatorProps> = ({ 
  isOpen, 
  onClose, 
  initialStatus = 'todo' 
}) => {
  const { addTask, tags: existingTags, createTag } = useTaskStore();
  const [input, setInput] = useState('');
  const [mode, setMode] = useState<'natural' | 'analysis'>('natural');
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [naturalLanguageResult, setNaturalLanguageResult] = useState<NaturalLanguageTask | null>(null);
  const [analysisResult, setAnalysisResult] = useState<TaskAnalysis | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleAnalyze = async () => {
    if (!input.trim()) return;

    setIsAnalyzing(true);
    setError(null);
    setNaturalLanguageResult(null);
    setAnalysisResult(null);

    try {
      if (mode === 'natural') {
        // 自然言語解析
        const result = await invoke('parse_natural_language_task', { 
          request: input.trim() 
        });
        setNaturalLanguageResult(result as NaturalLanguageTask);
      } else {
        // タスク分析
        const result = await invoke('analyze_task_with_ai', { 
          description: input.trim() 
        });
        setAnalysisResult(result as TaskAnalysis);
      }
    } catch (error) {
      console.error('Analysis failed:', error);
      setError('AI分析に失敗しました。Ollamaサーバーが起動していることを確認してください。');
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleCreateTask = async () => {
    try {
      if (mode === 'natural' && naturalLanguageResult) {
        // 自然言語結果からタスク作成
        const taskData = {
          title: naturalLanguageResult.title,
          description: naturalLanguageResult.description,
          status: naturalLanguageResult.suggested_status,
          dueDate: naturalLanguageResult.due_date_suggestion 
            ? new Date(naturalLanguageResult.due_date_suggestion) 
            : undefined,
          notificationSettings: naturalLanguageResult.notification_needed ? {
            notificationType: 'due_date_based' as const,
            level: 2 as const,
            daysBefore: 1,
          } : undefined,
          tags: await createTagsIfNeeded(naturalLanguageResult.tags),
        };

        await addTask(taskData);
        onClose();
        resetForm();
        
      } else if (mode === 'analysis' && analysisResult) {
        // 分析結果からタスク作成
        const taskData = {
          title: analysisResult.improved_title,
          description: analysisResult.improved_description,
          status: initialStatus,
          notificationSettings: {
            notificationType: 'due_date_based' as const,
            level: analysisResult.complexity === 'complex' ? 3 as const : 
                   analysisResult.complexity === 'medium' ? 2 as const : 1 as const,
            daysBefore: analysisResult.complexity === 'complex' ? 3 : 1,
          },
          tags: await createTagsIfNeeded(analysisResult.suggested_tags),
        };

        await addTask(taskData);

        // サブタスクも作成
        for (const subtask of analysisResult.subtasks) {
          await addTask({
            title: subtask.title,
            description: subtask.description,
            status: 'todo',
            tags: await createTagsIfNeeded(['subtask']),
          });
        }

        onClose();
        resetForm();
      }
    } catch (error) {
      console.error('Task creation failed:', error);
      setError('タスクの作成に失敗しました。');
    }
  };

  const createTagsIfNeeded = async (tagNames: string[]) => {
    const createdTags = [];
    
    for (const tagName of tagNames) {
      // 既存タグを探す
      const existingTag = existingTags.find(tag => 
        tag.name.toLowerCase() === tagName.toLowerCase()
      );
      
      if (existingTag) {
        createdTags.push(existingTag);
      } else {
        // 新しいタグを作成
        try {
          const newTag = await createTag({
            name: tagName,
            color: getRandomColor(),
          });
          createdTags.push(newTag);
        } catch (error) {
          console.error(`Failed to create tag: ${tagName}`, error);
        }
      }
    }
    
    return createdTags;
  };

  const getRandomColor = () => {
    const colors = ['#3b82f6', '#ef4444', '#10b981', '#f59e0b', '#8b5cf6', '#06b6d4'];
    return colors[Math.floor(Math.random() * colors.length)];
  };

  const resetForm = () => {
    setInput('');
    setNaturalLanguageResult(null);
    setAnalysisResult(null);
    setError(null);
  };

  const getComplexityIcon = (complexity: string) => {
    switch (complexity) {
      case 'simple': return '🟢';
      case 'medium': return '🟡';
      case 'complex': return '🔴';
      default: return '⚪';
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg w-full max-w-3xl max-h-[90vh] flex flex-col overflow-hidden">
        <div className="p-6 overflow-y-auto">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-xl font-semibold">🤖 AIタスク作成</h2>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600 text-xl"
            >
              ×
            </button>
          </div>

          {/* モード選択 */}
          <div className="mb-4">
            <div className="flex space-x-1 bg-gray-100 rounded-lg p-1">
              <button
                onClick={() => setMode('natural')}
                className={`flex-1 py-2 px-4 text-sm rounded-md transition-colors ${
                  mode === 'natural' 
                    ? 'bg-white text-blue-600 shadow-sm' 
                    : 'text-gray-600 hover:text-gray-800'
                }`}
              >
                💬 自然言語モード
              </button>
              <button
                onClick={() => setMode('analysis')}
                className={`flex-1 py-2 px-4 text-sm rounded-md transition-colors ${
                  mode === 'analysis' 
                    ? 'bg-white text-blue-600 shadow-sm' 
                    : 'text-gray-600 hover:text-gray-800'
                }`}
              >
                🔍 詳細分析モード
              </button>
            </div>
          </div>

          {/* 説明 */}
          <div className="mb-4 p-3 bg-blue-50 rounded-lg text-sm text-blue-800">
            {mode === 'natural' ? (
              <>
                <strong>自然言語モード:</strong> 「明日までにプレゼン資料を作成する」のような自然な文章でタスクを入力してください。
              </>
            ) : (
              <>
                <strong>詳細分析モード:</strong> タスクの概要を入力すると、AIが詳細に分析して改善提案やサブタスクを生成します。
              </>
            )}
          </div>

          {/* 入力エリア */}
          <div className="mb-6">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              {mode === 'natural' ? 'タスクの説明' : 'タスクの概要'}
            </label>
            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder={
                mode === 'natural' 
                  ? "例: 来週の会議用のプレゼン資料を金曜日までに作成したい"
                  : "例: Reactでタスク管理アプリを作る"
              }
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
            />
          </div>

          {/* 分析ボタン */}
          <div className="mb-6">
            <button
              onClick={handleAnalyze}
              disabled={!input.trim() || isAnalyzing}
              className="w-full py-3 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center justify-center space-x-2"
            >
              {isAnalyzing ? (
                <>
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  <span>AI分析中...</span>
                </>
              ) : (
                <>
                  <span>🤖</span>
                  <span>AI分析を実行</span>
                </>
              )}
            </button>
          </div>

          {/* エラー表示 */}
          {error && (
            <div className="mb-6 p-3 bg-red-50 border border-red-200 rounded-md text-red-700">
              {error}
            </div>
          )}

          {/* 自然言語結果 */}
          {naturalLanguageResult && (
            <div className="mb-6 p-4 bg-green-50 border border-green-200 rounded-md">
              <h3 className="font-semibold mb-3 text-green-800">📋 解析結果</h3>
              <div className="space-y-2 text-sm">
                <div><strong>タイトル:</strong> {naturalLanguageResult.title}</div>
                <div><strong>説明:</strong> {naturalLanguageResult.description}</div>
                <div><strong>推奨ステータス:</strong> {naturalLanguageResult.suggested_status}</div>
                {naturalLanguageResult.due_date_suggestion && (
                  <div><strong>期限:</strong> {naturalLanguageResult.due_date_suggestion}</div>
                )}
                <div><strong>タグ:</strong> {naturalLanguageResult.tags.join(', ')}</div>
                <div><strong>通知:</strong> {naturalLanguageResult.notification_needed ? '必要' : '不要'}</div>
              </div>
            </div>
          )}

          {/* 詳細分析結果 */}
          {analysisResult && (
            <div className="mb-6 p-4 bg-purple-50 border border-purple-200 rounded-md">
              <h3 className="font-semibold mb-3 text-purple-800">🔍 詳細分析結果</h3>
              <div className="space-y-3 text-sm">
                <div>
                  <strong>改善されたタイトル:</strong> {analysisResult.improved_title}
                </div>
                <div>
                  <strong>詳細説明:</strong> {analysisResult.improved_description}
                </div>
                <div className="flex items-center space-x-2">
                  <strong>複雑度:</strong>
                  <span className="flex items-center space-x-1">
                    <span>{getComplexityIcon(analysisResult.complexity)}</span>
                    <span>{analysisResult.complexity}</span>
                  </span>
                </div>
                <div>
                  <strong>見積もり時間:</strong> {analysisResult.estimated_hours}時間
                </div>
                <div>
                  <strong>推奨タグ:</strong> {analysisResult.suggested_tags.join(', ')}
                </div>
                
                {analysisResult.subtasks.length > 0 && (
                  <div>
                    <strong>サブタスク:</strong>
                    <ul className="mt-1 ml-4 space-y-1">
                      {analysisResult.subtasks.map((subtask, index) => (
                        <li key={index} className="text-xs">
                          • {subtask.title}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
                
                <div>
                  <strong>優先度の根拠:</strong> {analysisResult.priority_reasoning}
                </div>
              </div>
            </div>
          )}

          {/* アクションボタン */}
          {(naturalLanguageResult || analysisResult) && (
            <div className="flex justify-end space-x-3">
              <button
                onClick={resetForm}
                className="px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors"
              >
                リセット
              </button>
              <button
                onClick={handleCreateTask}
                className="px-6 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors"
              >
                {mode === 'analysis' && analysisResult?.subtasks && analysisResult.subtasks.length > 0 
                  ? `タスク+サブタスク(${analysisResult.subtasks.length}個)を作成` 
                  : 'タスクを作成'
                }
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
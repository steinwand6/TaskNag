import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LogService } from '../services/logService';

interface AIPersonality {
  id: string;
  name: string;
  description: string;
  tone_description: string;
  prompt_prefix: string;
  sample_phrases: string[];
  emoji_style: 'None' | 'Minimal' | 'Moderate' | 'Frequent';
}

interface PersonalitySelectorProps {
  onPersonalityChange?: (personality: AIPersonality | null) => void;
}

export const PersonalitySelector: React.FC<PersonalitySelectorProps> = ({ 
  onPersonalityChange 
}) => {
  const [personalities, setPersonalities] = useState<AIPersonality[]>([]);
  const [currentPersonality, setCurrentPersonality] = useState<{ name: string; description: string } | null>(null);
  const [selectedId, setSelectedId] = useState<string>('');
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isExpanded, setIsExpanded] = useState(false);

  // 性格アイコンマップ
  const getPersonalityIcon = (id: string): string => {
    switch (id) {
      case 'polite_secretary':
        return '👔';
      case 'friendly_colleague':
        return '👥';
      case 'enthusiastic_coach':
        return '💪';
      case 'caring_childhood_friend':
        return '🏠';
      default:
        return '🤖';
    }
  };

  // 性格カラーマップ
  const getPersonalityColor = (id: string): string => {
    switch (id) {
      case 'polite_secretary':
        return 'bg-blue-500';
      case 'friendly_colleague':
        return 'bg-green-500';
      case 'enthusiastic_coach':
        return 'bg-red-500';
      case 'caring_childhood_friend':
        return 'bg-purple-500';
      default:
        return 'bg-gray-500';
    }
  };

  // 初期化：利用可能な性格と現在の性格を取得
  useEffect(() => {
    const loadPersonalities = async () => {
      try {
        setIsLoading(true);
        
        // 利用可能な性格を取得
        const availablePersonalities = await invoke<AIPersonality[]>('get_available_personalities');
        setPersonalities(availablePersonalities);
        
        // 現在の性格を取得
        const current = await invoke<[string, string] | null>('get_current_personality');
        if (current) {
          setCurrentPersonality({ name: current[0], description: current[1] });
          // 現在の性格のIDを見つける
          const currentPersonality = availablePersonalities.find(p => p.name === current[0]);
          if (currentPersonality) {
            setSelectedId(currentPersonality.id);
          }
        }
        
        LogService.info('PersonalitySelector', `性格ロード完了: ${availablePersonalities.length}個`);
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : String(err);
        setError(errorMessage);
        LogService.error('PersonalitySelector', `性格ロードエラー: ${errorMessage}`);
      } finally {
        setIsLoading(false);
      }
    };

    loadPersonalities();
  }, []);

  // 性格変更処理
  const handlePersonalityChange = async (personalityId: string) => {
    try {
      await invoke('set_ai_personality', { personalityId });
      
      const selectedPersonality = personalities.find(p => p.id === personalityId);
      if (selectedPersonality) {
        setCurrentPersonality({
          name: selectedPersonality.name,
          description: selectedPersonality.description
        });
        setSelectedId(personalityId);
        onPersonalityChange?.(selectedPersonality);
        
        LogService.info('PersonalitySelector', `性格変更: ${selectedPersonality.name}`);
      }
      
      // 選択後は折りたたむ
      setIsExpanded(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      LogService.error('PersonalitySelector', `性格変更エラー: ${errorMessage}`);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-4">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-4">
        <p className="text-red-600 text-sm">AI性格の読み込みに失敗しました: {error}</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md p-4">
      {/* ヘッダー */}
      <div 
        className="flex items-center justify-between cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center space-x-3">
          <h3 className="text-lg font-semibold text-gray-800">AI性格設定</h3>
          {currentPersonality && (
            <div className="flex items-center space-x-2">
              <span className="text-2xl">{getPersonalityIcon(selectedId)}</span>
              <span className="text-sm font-medium text-gray-600">
                {currentPersonality.name}
              </span>
            </div>
          )}
        </div>
        <button
          className="p-1 hover:bg-gray-100 rounded-full transition-colors"
          aria-label={isExpanded ? '折りたたむ' : '展開する'}
        >
          <svg
            className={`w-5 h-5 text-gray-500 transform transition-transform ${
              isExpanded ? 'rotate-180' : ''
            }`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </button>
      </div>

      {/* 性格リスト（展開時） */}
      {isExpanded && (
        <div className="mt-4 space-y-3">
          {personalities.map((personality) => (
            <div
              key={personality.id}
              className={`border rounded-lg p-4 cursor-pointer transition-all ${
                selectedId === personality.id
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
              }`}
              onClick={() => handlePersonalityChange(personality.id)}
            >
              <div className="flex items-start space-x-3">
                {/* アイコン */}
                <div
                  className={`flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center text-white ${getPersonalityColor(
                    personality.id
                  )}`}
                >
                  <span className="text-xl">{getPersonalityIcon(personality.id)}</span>
                </div>

                {/* 情報 */}
                <div className="flex-grow">
                  <h4 className="font-semibold text-gray-800">{personality.name}</h4>
                  <p className="text-sm text-gray-600 mt-1">{personality.description}</p>
                  <p className="text-xs text-gray-500 mt-2">
                    口調: {personality.tone_description}
                  </p>

                  {/* サンプルフレーズ */}
                  {selectedId === personality.id && personality.sample_phrases.length > 0 && (
                    <div className="mt-3 p-3 bg-white rounded-md">
                      <p className="text-xs font-medium text-gray-700 mb-2">サンプルフレーズ:</p>
                      <div className="space-y-1">
                        {personality.sample_phrases.slice(0, 3).map((phrase, index) => (
                          <p key={index} className="text-xs text-gray-600 italic">
                            「{phrase}」
                          </p>
                        ))}
                      </div>
                    </div>
                  )}
                </div>

                {/* 選択インジケーター */}
                {selectedId === personality.id && (
                  <div className="flex-shrink-0">
                    <svg
                      className="w-6 h-6 text-blue-500"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                    >
                      <path
                        fillRule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                        clipRule="evenodd"
                      />
                    </svg>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* 現在の性格の説明（折りたたみ時） */}
      {!isExpanded && currentPersonality && (
        <div className="mt-3 p-3 bg-gray-50 rounded-md">
          <p className="text-sm text-gray-600">{currentPersonality.description}</p>
        </div>
      )}
    </div>
  );
};
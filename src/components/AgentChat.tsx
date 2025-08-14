import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

interface AgentChatProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AgentChat: React.FC<AgentChatProps> = ({ isOpen, onClose }) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [connectionChecked, setConnectionChecked] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const checkOllamaConnection = React.useCallback(async () => {
    try {
      const connected = await invoke('test_ollama_connection');
      setIsConnected(connected as boolean);
      
      if (connected) {
        // 初回メッセージ
        addMessage('assistant', 'こんにちは！TaskNagのAIアシスタントです。タスク管理について何でもお聞きください。');
      } else {
        addMessage('assistant', 'Ollamaサーバーに接続できませんでした。Ollamaが起動していることを確認してください。');
      }
    } catch (error) {
      console.error('Connection check failed:', error);
      setIsConnected(false);
      addMessage('assistant', 'Ollamaサーバーへの接続に失敗しました。サーバーが起動していることを確認してください。');
    } finally {
      setConnectionChecked(true);
    }
  }, []);

  // 初期接続チェック
  useEffect(() => {
    if (isOpen && !connectionChecked) {
      checkOllamaConnection();
    }
  }, [isOpen, connectionChecked, checkOllamaConnection]);

  // メッセージリストの自動スクロール
  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  // オープン時にフォーカス
  useEffect(() => {
    if (isOpen && inputRef.current) {
      setTimeout(() => inputRef.current?.focus(), 100);
    }
  }, [isOpen]);

  const addMessage = (role: 'user' | 'assistant', content: string) => {
    const newMessage: Message = {
      id: Date.now().toString(),
      role,
      content,
      timestamp: new Date(),
    };
    setMessages(prev => [...prev, newMessage]);
  };

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleSendMessage = async () => {
    if (!inputMessage.trim() || isLoading || !isConnected) return;

    const userMessage = inputMessage.trim();
    setInputMessage('');
    addMessage('user', userMessage);
    setIsLoading(true);

    try {
      const response = await invoke('chat_with_agent', {
        message: userMessage,
        context: null
      });
      
      addMessage('assistant', response as string);
    } catch (error) {
      console.error('Chat error:', error);
      addMessage('assistant', 'すみません、エラーが発生しました。もう一度お試しください。');
    } finally {
      setIsLoading(false);
      // フォーカスを戻す
      setTimeout(() => inputRef.current?.focus(), 100);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const handleClearChat = () => {
    setMessages([]);
    if (isConnected) {
      addMessage('assistant', 'チャット履歴をクリアしました。何かお聞きしたいことはありますか？');
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('ja-JP', { 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg w-full max-w-2xl h-[600px] flex flex-col">
        {/* ヘッダー */}
        <div className="flex justify-between items-center p-4 border-b border-gray-200">
          <div className="flex items-center space-x-3">
            <div className="w-3 h-3 rounded-full bg-blue-500"></div>
            <h2 className="text-lg font-semibold">🤖 AIエージェント</h2>
            <div className={`flex items-center space-x-1 text-xs ${
              isConnected ? 'text-green-600' : 'text-red-600'
            }`}>
              <div className={`w-2 h-2 rounded-full ${
                isConnected ? 'bg-green-500' : 'bg-red-500'
              }`}></div>
              <span>{isConnected ? 'オンライン' : 'オフライン'}</span>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={handleClearChat}
              className="text-gray-500 hover:text-gray-700 text-sm px-2 py-1 rounded transition-colors"
              disabled={messages.length === 0}
            >
              🗑️ クリア
            </button>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600 text-xl"
            >
              ×
            </button>
          </div>
        </div>

        {/* メッセージエリア */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {!connectionChecked ? (
            <div className="flex justify-center items-center h-full">
              <div className="text-gray-500">接続確認中...</div>
            </div>
          ) : messages.length === 0 ? (
            <div className="flex justify-center items-center h-full text-gray-500">
              メッセージを入力してチャットを開始してください
            </div>
          ) : (
            messages.map((message) => (
              <div key={message.id} className={`flex ${
                message.role === 'user' ? 'justify-end' : 'justify-start'
              }`}>
                <div className={`max-w-[80%] rounded-lg px-4 py-2 ${
                  message.role === 'user'
                    ? 'bg-blue-500 text-white'
                    : 'bg-gray-100 text-gray-800'
                }`}>
                  <div className="whitespace-pre-wrap break-words">
                    {message.content}
                  </div>
                  <div className={`text-xs mt-1 ${
                    message.role === 'user' ? 'text-blue-100' : 'text-gray-500'
                  }`}>
                    {formatTime(message.timestamp)}
                  </div>
                </div>
              </div>
            ))
          )}
          
          {isLoading && (
            <div className="flex justify-start">
              <div className="bg-gray-100 rounded-lg px-4 py-2">
                <div className="flex items-center space-x-2">
                  <div className="flex space-x-1">
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.1s' }}></div>
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                  </div>
                  <span className="text-gray-500 text-sm">考え中...</span>
                </div>
              </div>
            </div>
          )}
          
          <div ref={messagesEndRef} />
        </div>

        {/* 入力エリア */}
        <div className="border-t border-gray-200 p-4">
          <div className="flex space-x-2">
            <input
              ref={inputRef}
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder={isConnected ? "メッセージを入力..." : "Ollamaサーバーに接続してください"}
              className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
              disabled={!isConnected || isLoading}
            />
            <button
              onClick={handleSendMessage}
              disabled={!inputMessage.trim() || isLoading || !isConnected}
              className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isLoading ? '送信中...' : '送信'}
            </button>
          </div>
          
          {!isConnected && connectionChecked && (
            <div className="mt-2 text-sm text-red-600">
              💡 Ollamaサーバーを起動して、モデルをダウンロードしてください
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
import React from 'react';
import { TaskStatus } from './types/Task';
import { useTaskStore } from './stores/taskStore';
import { NewTaskModal } from './components/NewTaskModal';
import { Header } from './components/Header';
import { KanbanColumn } from './components/KanbanColumn';
import { ErrorMessage } from './components/ErrorMessage';
import { LoadingIndicator } from './components/LoadingIndicator';
import { TagManager } from './components/TagManager';
import { TaskFilter } from './components/TaskFilter';
import { AgentChat } from './components/AgentChat';
import { SmartTaskCreator } from './components/SmartTaskCreator';
import { STATUS_CONFIG, VISIBLE_STATUSES } from './constants';
import { useModal, useDragAndDrop, useNotifications } from './hooks';

import { LogService } from './services/logService';
import { NotificationService } from './services/notificationService';
import { listen } from '@tauri-apps/api/event';

function App() {
  const { getFilteredTasks, moveTask, loadTasks, loadTags, isLoading, error, selectedTags, searchQuery } = useTaskStore();
  
  // State for showing done tasks
  const [showDone, setShowDone] = React.useState(false);
  
  // State for tag manager modal
  const [showTagManager, setShowTagManager] = React.useState(false);
  
  // State for showing filter panel
  const [showFilters, setShowFilters] = React.useState(false);
  
  // State for AI features
  const [showAgentChat, setShowAgentChat] = React.useState(false);
  const [showSmartCreator, setShowSmartCreator] = React.useState(false);
  
  // Custom hooks
  const { isModalOpen, modalInitialStatus, openModal, closeModal } = useModal();
  const dragAndDropHandlers = useDragAndDrop(moveTask);
  useNotifications();

  // 音声再生関数
  const playNotificationSound = React.useCallback((level: number = 1) => {
    try {
      // Web Audio APIを使用して通知音を再生
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const oscillator = audioContext.createOscillator();
      const gainNode = audioContext.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(audioContext.destination);

      // レベルに応じた音の設定
      const frequency = level === 3 ? 800 : level === 2 ? 600 : 400;
      const duration = level === 3 ? 0.8 : 0.4;

      oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
      oscillator.type = 'sine';

      gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
      gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration);

      oscillator.start();
      oscillator.stop(audioContext.currentTime + duration);

      LogService.info(`通知音を再生しました (Level ${level})`);

      // レベル3の場合は複数回再生
      if (level === 3) {
        setTimeout(() => playNotificationSound(3), 500);
        setTimeout(() => playNotificationSound(3), 1000);
      }
    } catch (error) {
      LogService.error('通知音再生エラー:', error);
    }
  }, []);

  // Load tasks and tags on component mount
  React.useEffect(() => {
    LogService.info('アプリ', 'TaskNagアプリケーションが起動しました');
    loadTasks();
    loadTags();
    
    // 通知サービスを初期化
    NotificationService.initialize().catch(error => {
      LogService.error('通知サービス初期化エラー:', error);
    });

    // Tauriイベントリスナーを設定
    const setupEventListeners = async () => {
      try {
        // 通知音再生イベント
        await listen('play_notification_sound', (event) => {
          const { level } = event.payload as { level: number };
          playNotificationSound(level);
        });

        LogService.info('Tauriイベントリスナーを設定しました');
      } catch (error) {
        LogService.error('イベントリスナー設定エラー:', error);
      }
    };

    setupEventListeners();
  }, [loadTasks, loadTags, playNotificationSound]);

  // Get statuses to display based on showDone state
  const displayStatuses = showDone 
    ? [...VISIBLE_STATUSES, 'done' as TaskStatus]
    : VISIBLE_STATUSES;

  const getStatusData = (status: TaskStatus) => {
    const filteredTasks = getFilteredTasks();
    const statusTasks = filteredTasks.filter(task => task.status === status);
    
    return {
      ...STATUS_CONFIG[status],
      count: statusTasks.length,
      tasks: statusTasks,
    };
  };
  
  if (error) {
    return <ErrorMessage error={error} />;
  }
  
  return (
    <div className="min-h-screen bg-gray-50">
      <Header 
        isLoading={isLoading}
        onNewTask={() => openModal()}
        showDone={showDone}
        onToggleDone={() => setShowDone(!showDone)}
        onManageTags={() => setShowTagManager(true)}
        onToggleFilters={() => setShowFilters(!showFilters)}
        showFilters={showFilters}
        hasActiveFilters={selectedTags.length > 0 || searchQuery.trim().length > 0}
        onOpenAgentChat={() => setShowAgentChat(true)}
        onOpenSmartCreator={() => setShowSmartCreator(true)}
      />

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading && <LoadingIndicator />}
        
        {/* フィルタパネル */}
        {showFilters && (
          <div className="mb-6">
            <TaskFilter />
          </div>
        )}
        
        <div className={`grid grid-cols-1 gap-6 ${showDone ? 'md:grid-cols-4' : 'md:grid-cols-3'}`}>
          {displayStatuses.map((status) => {
            const statusData = getStatusData(status);
            
            return (
              <KanbanColumn
                key={status}
                status={status}
                statusData={statusData}
                isLoading={isLoading}
                isDragOver={dragAndDropHandlers.dragOverStatus === status}
                onMouseEnter={dragAndDropHandlers.handleColumnMouseEnter}
                onMouseLeave={dragAndDropHandlers.handleColumnMouseLeave}
                onClick={dragAndDropHandlers.handleColumnClick}
                onDragStart={dragAndDropHandlers.handleDragStart}
                onDragEnd={dragAndDropHandlers.handleDragEnd}
                onNewTask={openModal}
              />
            );
          })}
        </div>
      </main>

      {isModalOpen && (
        <NewTaskModal
          isOpen={isModalOpen}
          onClose={closeModal}
          initialStatus={modalInitialStatus}
        />
      )}

      <TagManager
        isOpen={showTagManager}
        onClose={() => setShowTagManager(false)}
      />

      <AgentChat
        isOpen={showAgentChat}
        onClose={() => setShowAgentChat(false)}
      />

      <SmartTaskCreator
        isOpen={showSmartCreator}
        onClose={() => setShowSmartCreator(false)}
      />

    </div>
  );
}

export default App;

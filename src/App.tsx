import React from 'react';
import { TaskStatus } from './types/Task';
import { useTaskStore } from './stores/taskStore';
import { NewTaskModal } from './components/NewTaskModal';
import { Header } from './components/Header';
import { KanbanColumn } from './components/KanbanColumn';
import { ErrorMessage } from './components/ErrorMessage';
import { LoadingIndicator } from './components/LoadingIndicator';
import { TagManager } from './components/TagManager';
import { STATUS_CONFIG, VISIBLE_STATUSES } from './constants';
import { useModal, useDragAndDrop, useNotifications } from './hooks';

import { LogService } from './services/logService';

function App() {
  const { getTasksByStatus, moveTask, loadTasks, loadTags, isLoading, error } = useTaskStore();
  
  // State for showing done tasks
  const [showDone, setShowDone] = React.useState(false);
  
  // State for tag manager modal
  const [showTagManager, setShowTagManager] = React.useState(false);
  
  // Custom hooks
  const { isModalOpen, modalInitialStatus, openModal, closeModal } = useModal();
  const dragAndDropHandlers = useDragAndDrop(moveTask);
  const { } = useNotifications();

  // Load tasks and tags on component mount
  React.useEffect(() => {
    LogService.info('アプリ', 'TaskNagアプリケーションが起動しました');
    loadTasks();
    loadTags();
  }, [loadTasks, loadTags]);

  // Get statuses to display based on showDone state
  const displayStatuses = showDone 
    ? [...VISIBLE_STATUSES, 'done' as TaskStatus]
    : VISIBLE_STATUSES;

  const getStatusData = (status: TaskStatus) => {
    const statusTasks = getTasksByStatus(status);
    
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
        onRefresh={loadTasks}
        showDone={showDone}
        onToggleDone={() => setShowDone(!showDone)}
        onManageTags={() => setShowTagManager(true)}
      />

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading && <LoadingIndicator />}
        
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

    </div>
  );
}

export default App;

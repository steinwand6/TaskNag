import React from 'react';
import { TaskStatus } from './types/Task';
import { useTaskStore } from './stores/taskStore';
import { NewTaskModal } from './components/NewTaskModal';
import { Header } from './components/Header';
import { KanbanColumn } from './components/KanbanColumn';
import { ErrorMessage } from './components/ErrorMessage';
import { LoadingIndicator } from './components/LoadingIndicator';
import { STATUS_CONFIG, TASK_STATUSES } from './constants';
import { useModal, useDragAndDrop } from './hooks';

function App() {
  const { getTasksByStatus, moveTask, loadTasks, isLoading, error } = useTaskStore();
  
  // Custom hooks
  const { isModalOpen, modalInitialStatus, openModal, closeModal } = useModal();
  const dragAndDropHandlers = useDragAndDrop(moveTask);

  // Load tasks on component mount
  React.useEffect(() => {
    loadTasks();
  }, [loadTasks]);

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
      />

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading && <LoadingIndicator />}
        
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          {TASK_STATUSES.map((status) => {
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
    </div>
  );
}

export default App;

import React from 'react';
import { TaskStatus } from './types/Task';
import { useTaskStore } from './stores/taskStore';
import { TaskCard } from './components/TaskCard';
import { NewTaskModal } from './components/NewTaskModal';

function App() {
  const { tasks, getTasksByStatus, moveTask, loadTasks, isLoading, error } = useTaskStore();
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const [modalInitialStatus, setModalInitialStatus] = React.useState<TaskStatus>('inbox');
  const [dragOverStatus, setDragOverStatus] = React.useState<TaskStatus | null>(null);
  const [draggingTaskId, setDraggingTaskId] = React.useState<string | null>(null);
  
  // Load tasks on component mount
  React.useEffect(() => {
    loadTasks();
  }, [loadTasks]);
  
  const handleNewTask = (status?: TaskStatus) => {
    setModalInitialStatus(status || 'inbox');
    setIsModalOpen(true);
  };

  const handleDragStart = (taskId: string) => {
    setDraggingTaskId(taskId);
  };

  const handleDragEnd = () => {
    setDraggingTaskId(null);
    setDragOverStatus(null);
  };

  const handleColumnMouseEnter = (status: TaskStatus) => {
    if (draggingTaskId) {
      setDragOverStatus(status);
    }
  };

  const handleColumnMouseLeave = () => {
    if (draggingTaskId) {
      setDragOverStatus(null);
    }
  };

  const handleColumnClick = (status: TaskStatus) => {
    if (draggingTaskId) {
      moveTask(draggingTaskId, status);
      setDraggingTaskId(null);
      setDragOverStatus(null);
    }
  };

  const getStatusData = (status: TaskStatus) => {
    const statusTasks = getTasksByStatus(status);
    const configs = {
      inbox: { title: '📥 INBOX', subtitle: '未分類', color: 'bg-slate-600' },
      todo: { title: '📋 TODO', subtitle: '実行予定', color: 'bg-blue-600' },
      in_progress: { title: '⚡ IN PROGRESS', subtitle: '実行中', color: 'bg-purple-600' },
      done: { title: '✅ DONE', subtitle: '完了', color: 'bg-green-600' },
    };
    
    return {
      ...configs[status],
      count: statusTasks.length,
      tasks: statusTasks,
    };
  };
  
  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <p className="font-bold">エラーが発生しました</p>
          <p>{error.message}</p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center">
              <h1 className="text-2xl font-bold text-gray-900">
                TaskNag 🗣️
              </h1>
              <p className="ml-3 text-sm text-gray-500">
                口うるさいタスク管理
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <button 
                onClick={() => handleNewTask()}
                className="btn-primary"
                disabled={isLoading}
              >
                + 新規タスク
              </button>
              <button 
                className="btn-secondary"
                onClick={() => loadTasks()}
                disabled={isLoading}
              >
                {isLoading ? '⏳' : '🔄'} 更新
              </button>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading && (
          <div className="mb-4 text-center">
            <div className="inline-flex items-center space-x-2">
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-500"></div>
              <span className="text-gray-600">読み込み中...</span>
            </div>
          </div>
        )}
        
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          {(['inbox', 'todo', 'in_progress', 'done'] as TaskStatus[]).map((status) => {
            const statusData = getStatusData(status);
            
            return (
              <div 
                key={status} 
                data-status={status}
                className={`bg-white rounded-lg shadow-sm border border-gray-200 transition-colors ${
                  dragOverStatus === status ? 'ring-2 ring-blue-400 border-blue-400 bg-blue-50' : ''
                }`}
                onMouseEnter={() => handleColumnMouseEnter(status)}
                onMouseLeave={handleColumnMouseLeave}
                onClick={() => handleColumnClick(status)}
              >
                <div className={`${statusData.color} text-white p-4 rounded-t-lg`}>
                  <h2 className="text-lg font-semibold">{statusData.title}</h2>
                  <p className="text-sm opacity-90">{statusData.subtitle} ({statusData.count})</p>
                </div>
                
                <div className="p-4 space-y-3">
                  <div className="min-h-[200px]">
                    {statusData.tasks.map((task) => (
                      <TaskCard 
                        key={task.id} 
                        task={task} 
                        onDragStart={handleDragStart}
                        onDragEnd={handleDragEnd}
                      />
                    ))}
                    
                    {statusData.tasks.length === 0 && !isLoading && (
                      <div className="text-center text-gray-400 py-8">
                        <p className="text-sm">タスクがありません</p>
                        <button 
                          onClick={() => handleNewTask(status)} 
                          className="text-blue-500 text-xs hover:underline mt-2"
                        >
                          + 新規追加
                        </button>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </main>

      {isModalOpen && (
        <NewTaskModal
          isOpen={isModalOpen}
          onClose={() => setIsModalOpen(false)}
          initialStatus={modalInitialStatus}
        />
      )}
    </div>
  );
}

export default App;

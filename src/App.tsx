import React from 'react';
import { TaskStatus } from './types/Task';
import { useTaskStore } from './stores/taskStore';
import { TaskCard } from './components/TaskCard';
import { NewTaskModal } from './components/NewTaskModal';

function App() {
  const { tasks, getTasksByStatus, moveTask } = useTaskStore();
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const [modalInitialStatus, setModalInitialStatus] = React.useState<TaskStatus>('inbox');
  const [dragOverStatus, setDragOverStatus] = React.useState<TaskStatus | null>(null);
  const [draggingTaskId, setDraggingTaskId] = React.useState<string | null>(null);
  
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
              >
                + 新規タスク
              </button>
              <button className="btn-secondary">
                ⚙️ 設定
              </button>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
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
                <div className={`${statusData.color} text-white px-4 py-3 rounded-t-lg`}>
                  <h2 className="font-semibold">{statusData.title}</h2>
                  <p className="text-sm opacity-90">{statusData.subtitle} ({statusData.count})</p>
                </div>
                <div 
                  className={`p-4 min-h-96 ${
                    dragOverStatus === status ? 'bg-blue-50/50' : ''
                  }`}
                >
                  <div className="space-y-3">
                    {statusData.tasks.map((task) => (
                      <TaskCard 
                        key={task.id} 
                        task={task} 
                        onDragStart={handleDragStart}
                        onDragEnd={handleDragEnd}
                      />
                    ))}
                  </div>
                  
                  {statusData.tasks.length === 0 && (
                    <div className="text-center text-gray-500 mt-8">
                      <p>タスクがありません</p>
                      {status === 'inbox' && (
                        <p className="text-sm mt-2">新しいタスクを追加してください</p>
                      )}
                      {status !== 'done' && (
                        <button
                          onClick={() => handleNewTask(status)}
                          className="mt-3 text-sm text-blue-600 hover:text-blue-800"
                        >
                          + タスクを追加
                        </button>
                      )}
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>

        {/* Welcome Message - Show only when no tasks exist */}
        {tasks.length === 0 && (
          <div className="mt-8 bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="text-center">
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                TaskNagへようこそ！ 🎉
              </h3>
              <p className="text-gray-600 mb-4">
                口うるさくて世話焼きなタスク管理アプリです。<br />
                あなたの生産性向上を全力でサポートします！
              </p>
              <div className="flex justify-center space-x-4">
                <button 
                  onClick={() => handleNewTask()}
                  className="btn-primary"
                >
                  最初のタスクを作成
                </button>
                <button className="btn-secondary">
                  使い方を見る
                </button>
              </div>
            </div>
          </div>
        )}
      </main>
      
      <NewTaskModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        initialStatus={modalInitialStatus}
      />
    </div>
  );
}

export default App;
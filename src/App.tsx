import React from 'react';
import { create } from 'zustand';

// Task status type
export type TaskStatus = 'inbox' | 'todo' | 'in_progress' | 'done';

// Task interface
export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  createdAt: Date;
  updatedAt: Date;
  priority: 'low' | 'medium' | 'high';
  dueDate?: Date;
}

// Zustand store interface
interface TaskStore {
  tasks: Task[];
  addTask: (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt'>) => void;
  updateTask: (id: string, updates: Partial<Task>) => void;
  deleteTask: (id: string) => void;
  moveTask: (id: string, newStatus: TaskStatus) => void;
  getTasksByStatus: (status: TaskStatus) => Task[];
}

// Create Zustand store
export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  
  addTask: (taskData) => {
    const newTask: Task = {
      ...taskData,
      id: crypto.randomUUID(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    set((state) => ({
      tasks: [...state.tasks, newTask],
    }));
  },
  
  updateTask: (id, updates) => {
    set((state) => ({
      tasks: state.tasks.map((task) =>
        task.id === id
          ? { ...task, ...updates, updatedAt: new Date() }
          : task
      ),
    }));
  },
  
  deleteTask: (id) => {
    set((state) => ({
      tasks: state.tasks.filter((task) => task.id !== id),
    }));
  },
  
  moveTask: (id, newStatus) => {
    set((state) => ({
      tasks: state.tasks.map((task) =>
        task.id === id
          ? { ...task, status: newStatus, updatedAt: new Date() }
          : task
      ),
    }));
  },
  
  getTasksByStatus: (status) => {
    return get().tasks.filter((task) => task.status === status);
  },
}));

// Task Card Component
const TaskCard: React.FC<{ task: Task }> = ({ task }) => {
  const { updateTask, deleteTask } = useTaskStore();
  const [editTask, setEditTask] = React.useState<Task | null>(null);
  
  const getPriorityColor = (priority: Task['priority']) => {
    switch (priority) {
      case 'high': return 'border-l-red-500';
      case 'medium': return 'border-l-yellow-500';
      case 'low': return 'border-l-green-500';
      default: return 'border-l-gray-300';
    }
  };
  
  const formatDate = (date?: Date) => {
    if (!date) return '';
    return new Intl.DateTimeFormat('ja-JP', {
      month: 'short',
      day: 'numeric',
    }).format(date);
  };

  const handleDoubleClick = () => {
    setEditTask(task);
  };
  
  return (
    <>
      <div 
        className={`bg-white p-3 rounded-lg border-l-4 ${getPriorityColor(task.priority)} shadow-sm hover:shadow-md transition-shadow cursor-pointer`}
        onDoubleClick={handleDoubleClick}
      >
        <div className="flex justify-between items-start mb-2">
          <h4 className="font-medium text-gray-900 text-sm leading-tight">
            {task.title}
          </h4>
          <button
            onClick={() => deleteTask(task.id)}
            className="text-gray-400 hover:text-red-500 text-xs ml-2"
          >
            ×
          </button>
        </div>
        
        {task.description && (
          <p className="text-gray-600 text-xs mb-2 line-clamp-2">
            {task.description}
          </p>
        )}
        
        <div className="flex justify-between items-center text-xs text-gray-500">
          <span className="capitalize">{task.priority}</span>
          {task.dueDate && (
            <span className="text-orange-600">
              📅 {formatDate(task.dueDate)}
            </span>
          )}
        </div>
      </div>
      
      <EditTaskModal
        isOpen={editTask !== null}
        onClose={() => setEditTask(null)}
        task={editTask}
      />
    </>
  );
};

// Edit Task Modal Component
const EditTaskModal: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  task: Task | null;
}> = ({ isOpen, onClose, task }) => {
  const { updateTask } = useTaskStore();
  const [formData, setFormData] = React.useState({
    title: '',
    description: '',
    priority: 'medium' as Task['priority'],
    status: 'inbox' as TaskStatus,
    dueDate: '',
  });
  
  // Initialize form data when task changes
  React.useEffect(() => {
    if (task) {
      setFormData({
        title: task.title,
        description: task.description || '',
        priority: task.priority,
        status: task.status,
        dueDate: task.dueDate ? task.dueDate.toISOString().split('T')[0] : '',
      });
    }
  }, [task]);
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.title.trim() || !task) return;
    
    updateTask(task.id, {
      title: formData.title,
      description: formData.description || undefined,
      priority: formData.priority,
      status: formData.status,
      dueDate: formData.dueDate ? new Date(formData.dueDate) : undefined,
    });
    
    onClose();
  };
  
  if (!isOpen || !task) return null;
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold">タスクを編集</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              タスク名 *
            </label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="タスクのタイトルを入力..."
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              説明
            </label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
              placeholder="タスクの詳細説明（任意）..."
            />
          </div>
          
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                優先度
              </label>
              <select
                value={formData.priority}
                onChange={(e) => setFormData({ ...formData, priority: e.target.value as Task['priority'] })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="low">低</option>
                <option value="medium">中</option>
                <option value="high">高</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ステータス
              </label>
              <select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value as TaskStatus })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="inbox">📥 INBOX</option>
                <option value="todo">📋 TODO</option>
                <option value="in_progress">⚡ IN PROGRESS</option>
                <option value="done">✅ DONE</option>
              </select>
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              期限
            </label>
            <input
              type="date"
              value={formData.dueDate}
              onChange={(e) => setFormData({ ...formData, dueDate: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-gray-600 hover:text-gray-800"
            >
              キャンセル
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              更新
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

// New Task Modal Component
const NewTaskModal: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  initialStatus?: TaskStatus;
}> = ({ isOpen, onClose, initialStatus = 'inbox' }) => {
  const { addTask } = useTaskStore();
  const [formData, setFormData] = React.useState({
    title: '',
    description: '',
    priority: 'medium' as Task['priority'],
    status: initialStatus,
    dueDate: '',
  });
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.title.trim()) return;
    
    addTask({
      title: formData.title,
      description: formData.description || undefined,
      priority: formData.priority,
      status: formData.status,
      dueDate: formData.dueDate ? new Date(formData.dueDate) : undefined,
    });
    
    setFormData({
      title: '',
      description: '',
      priority: 'medium',
      status: initialStatus,
      dueDate: '',
    });
    onClose();
  };
  
  if (!isOpen) return null;
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold">新規タスク作成</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              タスク名 *
            </label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="タスクのタイトルを入力..."
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              説明
            </label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
              placeholder="タスクの詳細説明（任意）..."
            />
          </div>
          
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                優先度
              </label>
              <select
                value={formData.priority}
                onChange={(e) => setFormData({ ...formData, priority: e.target.value as Task['priority'] })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="low">低</option>
                <option value="medium">中</option>
                <option value="high">高</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ステータス
              </label>
              <select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value as TaskStatus })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="inbox">📥 INBOX</option>
                <option value="todo">📋 TODO</option>
                <option value="in_progress">⚡ IN PROGRESS</option>
                <option value="done">✅ DONE</option>
              </select>
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              期限
            </label>
            <input
              type="date"
              value={formData.dueDate}
              onChange={(e) => setFormData({ ...formData, dueDate: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-gray-600 hover:text-gray-800"
            >
              キャンセル
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              作成
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

function App() {
  const { tasks, getTasksByStatus } = useTaskStore();
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const [modalInitialStatus, setModalInitialStatus] = React.useState<TaskStatus>('inbox');
  
  const handleNewTask = (status?: TaskStatus) => {
    setModalInitialStatus(status || 'inbox');
    setIsModalOpen(true);
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
              <div key={status} className="bg-white rounded-lg shadow-sm border border-gray-200">
                <div className={`${statusData.color} text-white px-4 py-3 rounded-t-lg`}>
                  <h2 className="font-semibold">{statusData.title}</h2>
                  <p className="text-sm opacity-90">{statusData.subtitle} ({statusData.count})</p>
                </div>
                <div className="p-4 min-h-96">
                  <div className="space-y-3">
                    {statusData.tasks.map((task) => (
                      <TaskCard key={task.id} task={task} />
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
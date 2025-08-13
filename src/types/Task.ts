// Task status type
export type TaskStatus = 'inbox' | 'todo' | 'in_progress' | 'done';

// Priority type
export type Priority = 'low' | 'medium' | 'high' | 'urgent';

// Task interface
export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: Priority;
  parentId?: string;
  dueDate?: Date;
  completedAt?: Date;
  createdAt: Date;
  updatedAt: Date;
}

// API Request interfaces
export interface CreateTaskRequest {
  title: string;
  description?: string;
  status: TaskStatus;
  priority: Priority;
  parentId?: string;
  dueDate?: Date;
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: Priority;
  parentId?: string;
  dueDate?: Date;
}

// Zustand store interface
export interface TaskStore {
  tasks: Task[];
  isLoading: boolean;
  error: Error | null;
  loadTasks: () => Promise<void>;
  addTask: (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt'>) => Promise<void>;
  updateTask: (id: string, updates: Partial<Task>) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  moveTask: (id: string, newStatus: TaskStatus) => Promise<void>;
  getTasksByStatus: (status: TaskStatus) => Task[];
}
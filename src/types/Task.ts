// Task status type
export type TaskStatus = 'inbox' | 'todo' | 'in_progress' | 'done';

// Priority type
export type Priority = 'low' | 'medium' | 'high' | 'required';

// Task interface
export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: Priority;
  parentId?: string;
  children?: Task[];
  dueDate?: Date;
  completedAt?: Date;
  createdAt: Date;
  updatedAt: Date;
  progress?: number; // 進捗率 (0-100)
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

// Notification types
export type NotificationLevel = 1 | 2 | 3;

export interface NotificationSettings {
  enabled: boolean;
  level1Days: number; // 期限当日 (0)
  level2Days: number; // 期限1日前 (1) 
  level3Days: number; // 期限3日前 (3)
}

export interface TaskNotification {
  taskId: string;
  title: string;
  level: NotificationLevel;
  daysUntilDue: number;
  priority: Priority;
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
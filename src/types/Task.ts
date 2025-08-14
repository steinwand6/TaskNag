// Task status type
export type TaskStatus = 'inbox' | 'todo' | 'in_progress' | 'done';

// Priority type REMOVED as per .kiro/specs/notification-system-redesign
// Individual notification settings replace the priority system
// 新しい通知設定の型定義
export interface TaskNotificationSettings {
  notificationType: 'none' | 'due_date_based' | 'recurring';
  daysBefore?: number;        // 期日何日前から (1-30)
  notificationTime?: string;  // HH:MM形式
  daysOfWeek?: number[];      // 0=日曜, 1=月曜...6=土曜
  level: 1 | 2 | 3;          // 通知レベル
}

export interface TaskNotification {
  taskId: string;
  title: string;
  level: 1 | 2 | 3;
  daysUntilDue?: number;
  notificationType: string;
}

// Task interface
export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  // priority field REMOVED as per .kiro/specs/notification-system-redesign
  parentId?: string;
  children?: Task[];
  dueDate?: Date;
  completedAt?: Date;
  createdAt: Date;
  updatedAt: Date;
  progress?: number; // 進捗率 (0-100)
  // 新しい通知設定フィールド
  notificationSettings?: TaskNotificationSettings;
}

// API Request interfaces
export interface CreateTaskRequest {
  title: string;
  description?: string;
  status: TaskStatus;
  // priority field REMOVED as per .kiro/specs/notification-system-redesign
  parentId?: string;
  dueDate?: Date;
  notificationSettings?: TaskNotificationSettings;
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  // priority field REMOVED as per .kiro/specs/notification-system-redesign
  parentId?: string;
  dueDate?: Date;
  notificationSettings?: TaskNotificationSettings;
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
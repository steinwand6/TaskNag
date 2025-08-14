// Task status type
export type TaskStatus = 'inbox' | 'todo' | 'in_progress' | 'done';

// Tag interfaces
export interface Tag {
  id: string;
  name: string;
  color: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface CreateTagRequest {
  name: string;
  color: string;
}

export interface UpdateTagRequest {
  name?: string;
  color?: string;
}

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
  // タグシステム
  tags?: Tag[];
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
  tags?: Tag[];
}

// Zustand store interface
export interface TaskStore {
  tasks: Task[];
  tags: Tag[];
  isLoading: boolean;
  error: Error | null;
  // フィルタリング状態
  selectedTags: string[]; // 選択中のタグID
  searchQuery: string;
  showCompletedTasks: boolean;
  
  loadTasks: () => Promise<void>;
  addTask: (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt'>) => Promise<void>;
  updateTask: (id: string, updates: Partial<Task>) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  moveTask: (id: string, newStatus: TaskStatus) => Promise<void>;
  getTasksByStatus: (status: TaskStatus) => Task[];
  
  // フィルタリング機能
  getFilteredTasks: () => Task[];
  setSelectedTags: (tagIds: string[]) => void;
  toggleTag: (tagId: string) => void;
  clearTagFilter: () => void;
  setSearchQuery: (query: string) => void;
  setShowCompletedTasks: (show: boolean) => void;
  
  // タグ関連の操作
  loadTags: () => Promise<void>;
  createTag: (tag: CreateTagRequest) => Promise<Tag>;
  updateTag: (id: string, updates: UpdateTagRequest) => Promise<Tag>;
  deleteTag: (id: string) => Promise<void>;
  addTagToTask: (taskId: string, tagId: string) => Promise<void>;
  removeTagFromTask: (taskId: string, tagId: string) => Promise<void>;
  getTagsForTask: (taskId: string) => Promise<Tag[]>;
}
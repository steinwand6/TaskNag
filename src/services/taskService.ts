import { invoke } from '@tauri-apps/api/core';
import { Task, TaskStatus, CreateTaskRequest, UpdateTaskRequest, TaskNotification, TaskNotificationSettings, Tag, CreateTagRequest, UpdateTagRequest } from '../types/Task';

export class TaskService {
  static async createTask(request: CreateTaskRequest): Promise<Task> {
    const task = await invoke('create_task', { request });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async getTasks(): Promise<Task[]> {
    const tasks = await invoke<any[]>('get_tasks');
    return tasks.map((task: any) => this.mapTaskWithNotificationSettings(task));
  }

  static async getTaskById(id: string): Promise<Task> {
    const task = await invoke('get_task_by_id', { id });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async updateTask(id: string, request: UpdateTaskRequest): Promise<Task> {
    const task = await invoke('update_task', { id, request });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async deleteTask(id: string): Promise<void> {
    return await invoke('delete_task', { id });
  }

  static async getTasksByStatus(status: TaskStatus): Promise<Task[]> {
    const tasks = await invoke<any[]>('get_tasks_by_status', { status });
    return tasks.map((task: any) => this.mapTaskWithNotificationSettings(task));
  }

  static async moveTask(id: string, newStatus: TaskStatus): Promise<Task> {
    const task = await invoke('move_task', { id, newStatus });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async getIncompleteTaskCount(): Promise<number> {
    return await invoke('get_incomplete_task_count');
  }

  static async updateTrayTitle(): Promise<void> {
    return await invoke('update_tray_title');
  }


  static async updateTaskNotificationSettings(
    id: string, 
    settings: TaskNotificationSettings
  ): Promise<Task> {
    const task = await invoke('update_task_notification_settings', { 
      id, 
      notificationSettings: settings 
    });
    return this.mapTaskWithNotificationSettings(task);
  }

  // 子タスク管理機能
  static async getChildren(parentId: string): Promise<Task[]> {
    const tasks = await invoke<any[]>('get_children', { parentId });
    return tasks.map((task: any) => this.mapTaskWithNotificationSettings(task));
  }

  static async getTaskWithChildren(id: string): Promise<Task> {
    const task = await invoke('get_task_with_children', { id });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async updateProgress(id: string, progress: number): Promise<Task> {
    const task = await invoke('update_progress', { id, progress });
    return this.mapTaskWithNotificationSettings(task);
  }

  static async calculateAndUpdateProgress(parentId: string): Promise<number> {
    return await invoke('calculate_and_update_progress', { parentId });
  }

  static async getRootTasks(): Promise<Task[]> {
    const tasks = await invoke<any[]>('get_root_tasks');
    return tasks.map((task: any) => this.mapTaskWithNotificationSettings(task));
  }

  // タグ関連操作
  static async getAllTags(): Promise<Tag[]> {
    const tags = await invoke<any[]>('get_all_tags');
    return tags.map((tag: any) => this.mapTag(tag));
  }

  static async getTagById(id: string): Promise<Tag> {
    const tag = await invoke('get_tag_by_id', { id });
    return this.mapTag(tag);
  }

  static async createTag(request: CreateTagRequest): Promise<Tag> {
    const tag = await invoke('create_tag', { request });
    return this.mapTag(tag);
  }

  static async updateTag(id: string, request: UpdateTagRequest): Promise<Tag> {
    const tag = await invoke('update_tag', { id, request });
    return this.mapTag(tag);
  }

  static async deleteTag(id: string): Promise<void> {
    return await invoke('delete_tag', { id });
  }

  static async addTagToTask(taskId: string, tagId: string): Promise<void> {
    return await invoke('add_tag_to_task', { taskId, tagId });
  }

  static async removeTagFromTask(taskId: string, tagId: string): Promise<void> {
    return await invoke('remove_tag_from_task', { taskId, tagId });
  }

  static async getTagsForTask(taskId: string): Promise<Tag[]> {
    const tags = await invoke<any[]>('get_tags_for_task', { taskId });
    return tags.map((tag: any) => this.mapTag(tag));
  }

  private static mapTag(tag: any): Tag {
    return {
      id: tag.id,
      name: tag.name,
      color: tag.color,
      createdAt: new Date(tag.createdAt || tag.created_at),
      updatedAt: new Date(tag.updatedAt || tag.updated_at),
    };
  }

  private static mapTaskWithNotificationSettings(task: any): Task {
    
    // 通知設定フィールドをTaskNotificationSettingsオブジェクトに変換
    const notificationSettings: TaskNotificationSettings = {
      notificationType: task.notificationType || 'none',
      daysBefore: task.notificationDaysBefore,
      notificationTime: task.notificationTime,
      daysOfWeek: task.notificationDaysOfWeek 
        ? JSON.parse(task.notificationDaysOfWeek) 
        : undefined,
      level: task.notificationLevel || 1,
    };

    return {
      ...task,
      notificationSettings: notificationSettings.notificationType !== 'none' ? notificationSettings : undefined,
      // 日付フィールドの変換
      dueDate: task.dueDate || task.due_date ? new Date(task.dueDate || task.due_date) : undefined,
      completedAt: task.completedAt || task.completed_at ? new Date(task.completedAt || task.completed_at) : undefined,
      createdAt: new Date(task.createdAt || task.created_at),
      updatedAt: new Date(task.updatedAt || task.updated_at),
    };
  }
}
import { invoke } from '@tauri-apps/api/core';
import { Task, TaskStatus, CreateTaskRequest, UpdateTaskRequest, TaskNotification } from '../types/Task';

export class TaskService {
  static async createTask(request: CreateTaskRequest): Promise<Task> {
    return await invoke('create_task', { request });
  }

  static async getTasks(): Promise<Task[]> {
    return await invoke('get_tasks');
  }

  static async getTaskById(id: string): Promise<Task> {
    return await invoke('get_task_by_id', { id });
  }

  static async updateTask(id: string, request: UpdateTaskRequest): Promise<Task> {
    return await invoke('update_task', { id, request });
  }

  static async deleteTask(id: string): Promise<void> {
    return await invoke('delete_task', { id });
  }

  static async getTasksByStatus(status: TaskStatus): Promise<Task[]> {
    return await invoke('get_tasks_by_status', { status });
  }

  static async moveTask(id: string, newStatus: TaskStatus): Promise<Task> {
    return await invoke('move_task', { id, newStatus });
  }

  static async getIncompleteTaskCount(): Promise<number> {
    return await invoke('get_incomplete_task_count');
  }

  static async updateTrayTitle(): Promise<void> {
    return await invoke('update_tray_title');
  }

  static async checkNotifications(): Promise<TaskNotification[]> {
    return await invoke('check_notifications');
  }

  // 子タスク管理機能
  static async getChildren(parentId: string): Promise<Task[]> {
    return await invoke('get_children', { parentId });
  }

  static async getTaskWithChildren(id: string): Promise<Task> {
    return await invoke('get_task_with_children', { id });
  }

  static async updateProgress(id: string, progress: number): Promise<Task> {
    return await invoke('update_progress', { id, progress });
  }

  static async calculateAndUpdateProgress(parentId: string): Promise<number> {
    return await invoke('calculate_and_update_progress', { parentId });
  }

  static async getRootTasks(): Promise<Task[]> {
    return await invoke('get_root_tasks');
  }
}
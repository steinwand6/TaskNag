import { invoke } from '@tauri-apps/api/core';
import { Task, TaskStatus, CreateTaskRequest, UpdateTaskRequest } from '../types/Task';

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
}
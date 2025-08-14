import { create } from 'zustand';
import { TaskStore, CreateTaskRequest, UpdateTaskRequest } from '../types/Task';
import { TaskService } from '../services/taskService';
import { LogService } from '../services/logService';

// Create Zustand store
export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  isLoading: false,
  error: null,
  
  // Load root tasks from backend (tasks without parent)
  loadTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const tasks = await TaskService.getRootTasks();
      // Convert date strings to Date objects
      const parsedTasks = tasks.map(task => ({
        ...task,
        createdAt: new Date(task.createdAt),
        updatedAt: new Date(task.updatedAt),
        completedAt: task.completedAt ? new Date(task.completedAt) : undefined,
        dueDate: task.dueDate ? new Date(task.dueDate) : undefined,
      }));
      set({ tasks: parsedTasks, isLoading: false });
      // Update system tray title
      TaskService.updateTrayTitle().catch(console.error);
    } catch (error) {
      LogService.error('TaskStore.loadTasks error', error);
      set({ error: error as Error, isLoading: false });
    }
  },
  
  addTask: async (taskData) => {
    set({ isLoading: true, error: null });
    try {
      const createRequest: CreateTaskRequest = {
        title: taskData.title,
        description: taskData.description,
        status: taskData.status,
        priority: taskData.priority,
        parentId: taskData.parentId,
        dueDate: taskData.dueDate?.toISOString(),
        notificationSettings: taskData.notificationSettings,
      };

      const newTask = await TaskService.createTask(createRequest);
      const parsedTask = {
        ...newTask,
        createdAt: new Date(newTask.createdAt),
        updatedAt: new Date(newTask.updatedAt),
        completedAt: newTask.completedAt ? new Date(newTask.completedAt) : undefined,
        dueDate: newTask.dueDate ? new Date(newTask.dueDate) : undefined,
      };

      // Only add to state if it's a root task (no parent)
      if (!taskData.parentId) {
        set(state => ({
          tasks: [...state.tasks, parsedTask],
          isLoading: false,
        }));
      } else {
        set({ isLoading: false });
      }
      
      // Update system tray title
      TaskService.updateTrayTitle().catch(console.error);
    } catch (error) {
      console.error('TaskStore.addTask error:', error);
      set({ error: error as Error, isLoading: false });
    }
  },
  
  updateTask: async (id, updateData) => {
    set({ isLoading: true, error: null });
    try {
      const updateRequest: UpdateTaskRequest = {
        title: updateData.title,
        description: updateData.description,
        status: updateData.status,
        priority: updateData.priority,
        parentId: updateData.parentId,
        dueDate: updateData.dueDate?.toISOString(),
        notificationSettings: updateData.notificationSettings,
      };

      const updatedTask = await TaskService.updateTask(id, updateRequest);
      const parsedTask = {
        ...updatedTask,
        createdAt: new Date(updatedTask.createdAt),
        updatedAt: new Date(updatedTask.updatedAt),
        completedAt: updatedTask.completedAt ? new Date(updatedTask.completedAt) : undefined,
        dueDate: updatedTask.dueDate ? new Date(updatedTask.dueDate) : undefined,
      };

      set(state => ({
        tasks: state.tasks.map(task => task.id === id ? parsedTask : task),
        isLoading: false,
      }));
      
      // Update system tray title
      TaskService.updateTrayTitle().catch(console.error);
    } catch (error) {
      console.error('TaskStore.updateTask error:', error);
      set({ error: error as Error, isLoading: false });
    }
  },
  
  deleteTask: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.deleteTask(id);
      set(state => ({
        tasks: state.tasks.filter(task => task.id !== id),
        isLoading: false,
      }));
      
      // Update system tray title
      TaskService.updateTrayTitle().catch(console.error);
    } catch (error) {
      console.error('TaskStore.deleteTask error:', error);
      set({ error: error as Error, isLoading: false });
    }
  },
  
  moveTask: async (taskId, newStatus) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.moveTask(taskId, newStatus);
      
      // Find the task and update its status
      const state = get();
      const taskToUpdate = state.tasks.find(task => task.id === taskId);
      if (taskToUpdate) {
        const updatedTask = { ...taskToUpdate, status: newStatus };
        set({
          tasks: state.tasks.map(task => task.id === taskId ? updatedTask : task),
          isLoading: false,
        });
      } else {
        set({ isLoading: false });
      }
      
      // Update system tray title
      TaskService.updateTrayTitle().catch(console.error);
    } catch (error) {
      console.error('TaskStore.moveTask error:', error);
      set({ error: error as Error, isLoading: false });
    }
  },

  // Get tasks by status
  getTasksByStatus: (status) => {
    return get().tasks.filter(task => task.status === status);
  },
}));
import { create } from 'zustand';
import { TaskStore, CreateTaskRequest, UpdateTaskRequest } from '../types/Task';
import { TaskService } from '../services/taskService';

// Create Zustand store
export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  isLoading: false,
  error: null,
  
  // Load tasks from backend
  loadTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const tasks = await TaskService.getTasks();
      // Convert date strings to Date objects
      const parsedTasks = tasks.map(task => ({
        ...task,
        createdAt: new Date(task.createdAt),
        updatedAt: new Date(task.updatedAt),
        completedAt: task.completedAt ? new Date(task.completedAt) : undefined,
        dueDate: task.dueDate ? new Date(task.dueDate) : undefined,
      }));
      set({ tasks: parsedTasks, isLoading: false });
    } catch (error) {
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
        dueDate: taskData.dueDate,
      };
      const newTask = await TaskService.createTask(createRequest);
      const parsedTask = {
        ...newTask,
        createdAt: new Date(newTask.createdAt),
        updatedAt: new Date(newTask.updatedAt),
        completedAt: newTask.completedAt ? new Date(newTask.completedAt) : undefined,
        dueDate: newTask.dueDate ? new Date(newTask.dueDate) : undefined,
      };
      set((state) => ({
        tasks: [...state.tasks, parsedTask],
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as Error, isLoading: false });
    }
  },
  
  updateTask: async (id, updates) => {
    set({ isLoading: true, error: null });
    try {
      const updateRequest: UpdateTaskRequest = {
        title: updates.title,
        description: updates.description,
        status: updates.status,
        priority: updates.priority,
        parentId: updates.parentId,
        dueDate: updates.dueDate,
      };
      const updatedTask = await TaskService.updateTask(id, updateRequest);
      const parsedTask = {
        ...updatedTask,
        createdAt: new Date(updatedTask.createdAt),
        updatedAt: new Date(updatedTask.updatedAt),
        completedAt: updatedTask.completedAt ? new Date(updatedTask.completedAt) : undefined,
        dueDate: updatedTask.dueDate ? new Date(updatedTask.dueDate) : undefined,
      };
      set((state) => ({
        tasks: state.tasks.map((task) =>
          task.id === id ? parsedTask : task
        ),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as Error, isLoading: false });
    }
  },
  
  deleteTask: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.deleteTask(id);
      set((state) => ({
        tasks: state.tasks.filter((task) => task.id !== id),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as Error, isLoading: false });
    }
  },
  
  moveTask: async (id, newStatus) => {
    set({ isLoading: true, error: null });
    try {
      const updatedTask = await TaskService.moveTask(id, newStatus);
      const parsedTask = {
        ...updatedTask,
        createdAt: new Date(updatedTask.createdAt),
        updatedAt: new Date(updatedTask.updatedAt),
        completedAt: updatedTask.completedAt ? new Date(updatedTask.completedAt) : undefined,
        dueDate: updatedTask.dueDate ? new Date(updatedTask.dueDate) : undefined,
      };
      set((state) => ({
        tasks: state.tasks.map((task) =>
          task.id === id ? parsedTask : task
        ),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as Error, isLoading: false });
    }
  },
  
  getTasksByStatus: (status) => {
    return get().tasks.filter((task) => task.status === status);
  },
}));
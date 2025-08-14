import { create } from 'zustand';
import { TaskStore, CreateTaskRequest, UpdateTaskRequest } from '../types/Task';
import { TaskService } from '../services/taskService';
import { LogService } from '../services/logService';

// Create Zustand store
export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  tags: [],
  isLoading: false,
  error: null,
  
  // フィルタリング状態
  selectedTags: [],
  searchQuery: '',
  showCompletedTasks: true,
  
  // Load root tasks from backend (tasks without parent)
  loadTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const tasks = await TaskService.getRootTasks();
      
      // タスクごとにタグ情報を取得
      const tasksWithTags = await Promise.all(
        tasks.map(async (task) => {
          try {
            const tags = await TaskService.getTagsForTask(task.id);
            return {
              ...task,
              tags: tags.map(tag => ({
                ...tag,
                createdAt: new Date(tag.createdAt),
                updatedAt: new Date(tag.updatedAt),
              })),
            };
          } catch (error) {
            LogService.error(`Failed to load tags for task ${task.id}`, error);
            return { ...task, tags: [] };
          }
        })
      );
      
      // Convert date strings to Date objects
      const parsedTasks = tasksWithTags.map(task => ({
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
        parentId: taskData.parentId,
        dueDate: taskData.dueDate,
        notificationSettings: taskData.notificationSettings,
        tags: taskData.tags,
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
      // デバッグ: 送信されるタグデータをログ出力
      const logMessage = `updateTask - updateData.tags: ${JSON.stringify(updateData.tags?.map(tag => ({ id: tag.id, name: tag.name, color: tag.color })))}`;
      console.log(logMessage);
      LogService.info(`Frontend updateTask: ${logMessage}`);
      
      const updateRequest: UpdateTaskRequest = {
        title: updateData.title,
        description: updateData.description,
        status: updateData.status,
        parentId: updateData.parentId,
        dueDate: updateData.dueDate,
        notificationSettings: updateData.notificationSettings,
        tags: updateData.tags,
      };

      LogService.info(`Frontend updateTask: Calling TaskService.updateTask with request: ${JSON.stringify(updateRequest, null, 2)}`);
      const updatedTask = await TaskService.updateTask(id, updateRequest);
      LogService.info(`Frontend updateTask: Successfully updated task ${id}`);
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
      const errorMessage = `TaskStore.updateTask error: ${error}`;
      console.error(errorMessage);
      LogService.error(`Frontend updateTask ERROR: ${errorMessage}`);
      LogService.error(`Frontend updateTask ERROR - Full error object: ${JSON.stringify(error, null, 2)}`);
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

  // タグ関連操作
  loadTags: async () => {
    set({ isLoading: true, error: null });
    try {
      const tags = await TaskService.getAllTags();
      const parsedTags = tags.map(tag => ({
        ...tag,
        createdAt: new Date(tag.createdAt),
        updatedAt: new Date(tag.updatedAt),
      }));
      set({ tags: parsedTags, isLoading: false });
    } catch (error) {
      LogService.error('TaskStore.loadTags error', error);
      set({ error: error as Error, isLoading: false });
    }
  },

  createTag: async (tagData) => {
    set({ isLoading: true, error: null });
    try {
      const newTag = await TaskService.createTag(tagData);
      const parsedTag = {
        ...newTag,
        createdAt: new Date(newTag.createdAt),
        updatedAt: new Date(newTag.updatedAt),
      };
      set(state => ({
        tags: [...state.tags, parsedTag],
        isLoading: false,
      }));
      return parsedTag;
    } catch (error) {
      LogService.error('TaskStore.createTag error', error);
      set({ error: error as Error, isLoading: false });
      throw error;
    }
  },

  updateTag: async (id, updateData) => {
    set({ isLoading: true, error: null });
    try {
      const updatedTag = await TaskService.updateTag(id, updateData);
      const parsedTag = {
        ...updatedTag,
        createdAt: new Date(updatedTag.createdAt),
        updatedAt: new Date(updatedTag.updatedAt),
      };
      set(state => ({
        tags: state.tags.map(tag => tag.id === id ? parsedTag : tag),
        isLoading: false,
      }));
      return parsedTag;
    } catch (error) {
      LogService.error('TaskStore.updateTag error', error);
      set({ error: error as Error, isLoading: false });
      throw error;
    }
  },

  deleteTag: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.deleteTag(id);
      set(state => ({
        tags: state.tags.filter(tag => tag.id !== id),
        isLoading: false,
      }));
    } catch (error) {
      LogService.error('TaskStore.deleteTag error', error);
      set({ error: error as Error, isLoading: false });
    }
  },

  addTagToTask: async (taskId, tagId) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.addTagToTask(taskId, tagId);
      // タスクのタグ情報を更新するため、そのタスクのタグを再取得
      const taskTags = await TaskService.getTagsForTask(taskId);
      const parsedTags = taskTags.map(tag => ({
        ...tag,
        createdAt: new Date(tag.createdAt),
        updatedAt: new Date(tag.updatedAt),
      }));
      
      set(state => ({
        tasks: state.tasks.map(task => 
          task.id === taskId ? { ...task, tags: parsedTags } : task
        ),
        isLoading: false,
      }));
    } catch (error) {
      LogService.error('TaskStore.addTagToTask error', error);
      set({ error: error as Error, isLoading: false });
    }
  },

  removeTagFromTask: async (taskId, tagId) => {
    set({ isLoading: true, error: null });
    try {
      await TaskService.removeTagFromTask(taskId, tagId);
      // タスクのタグ情報を更新するため、そのタスクのタグを再取得
      const taskTags = await TaskService.getTagsForTask(taskId);
      const parsedTags = taskTags.map(tag => ({
        ...tag,
        createdAt: new Date(tag.createdAt),
        updatedAt: new Date(tag.updatedAt),
      }));
      
      set(state => ({
        tasks: state.tasks.map(task => 
          task.id === taskId ? { ...task, tags: parsedTags } : task
        ),
        isLoading: false,
      }));
    } catch (error) {
      LogService.error('TaskStore.removeTagFromTask error', error);
      set({ error: error as Error, isLoading: false });
    }
  },

  getTagsForTask: async (taskId) => {
    try {
      const tags = await TaskService.getTagsForTask(taskId);
      return tags.map(tag => ({
        ...tag,
        createdAt: new Date(tag.createdAt),
        updatedAt: new Date(tag.updatedAt),
      }));
    } catch (error) {
      LogService.error('TaskStore.getTagsForTask error', error);
      throw error;
    }
  },

  // フィルタリング機能
  getFilteredTasks: () => {
    const { tasks, selectedTags, searchQuery, showCompletedTasks } = get();
    
    return tasks.filter(task => {
      // 完了タスクフィルタ
      if (!showCompletedTasks && task.status === 'done') {
        return false;
      }
      
      // タグフィルタ
      if (selectedTags.length > 0) {
        const taskTagIds = task.tags?.map(tag => tag.id) || [];
        const hasSelectedTag = selectedTags.some(selectedTagId => 
          taskTagIds.includes(selectedTagId)
        );
        if (!hasSelectedTag) {
          return false;
        }
      }
      
      // 検索クエリフィルタ
      if (searchQuery.trim()) {
        const query = searchQuery.toLowerCase();
        const titleMatch = task.title.toLowerCase().includes(query);
        const descriptionMatch = task.description?.toLowerCase().includes(query) || false;
        const tagMatch = task.tags?.some(tag => 
          tag.name.toLowerCase().includes(query)
        ) || false;
        
        if (!titleMatch && !descriptionMatch && !tagMatch) {
          return false;
        }
      }
      
      return true;
    });
  },

  setSelectedTags: (tagIds) => {
    set({ selectedTags: tagIds });
  },

  toggleTag: (tagId) => {
    set(state => ({
      selectedTags: state.selectedTags.includes(tagId)
        ? state.selectedTags.filter(id => id !== tagId)
        : [...state.selectedTags, tagId]
    }));
  },

  clearTagFilter: () => {
    set({ selectedTags: [] });
  },

  setSearchQuery: (query) => {
    set({ searchQuery: query });
  },

  setShowCompletedTasks: (show) => {
    set({ showCompletedTasks: show });
  },
}));
import { TaskStatus } from '../types/Task';

// Task statuses in order
export const TASK_STATUSES: TaskStatus[] = ['inbox', 'todo', 'in_progress', 'done'];

// Visible statuses (excluding done for main UI)
export const VISIBLE_STATUSES: TaskStatus[] = ['inbox', 'todo', 'in_progress'];

// Default task status
export const DEFAULT_TASK_STATUS: TaskStatus = 'inbox';

// Status configuration
export const STATUS_CONFIG = {
  inbox: {
    title: '📥 INBOX',
    subtitle: '未分類',
    color: 'bg-slate-600',
  },
  todo: {
    title: '📋 TODO',
    subtitle: '実行予定',
    color: 'bg-blue-600',
  },
  in_progress: {
    title: '⚡ IN PROGRESS',
    subtitle: '実行中',
    color: 'bg-purple-600',
  },
  done: {
    title: '✅ DONE',
    subtitle: '完了',
    color: 'bg-green-600',
  },
} as const;

// Status options for select components
export const STATUS_OPTIONS = [
  { value: 'inbox', label: '📥 INBOX' },
  { value: 'todo', label: '📋 TODO' },
  { value: 'in_progress', label: '⚡ IN PROGRESS' },
  { value: 'done', label: '✅ DONE' },
] as const;
import { Priority } from '../types/Task';

// Default priority
export const DEFAULT_PRIORITY: Priority = 'medium';

// Priority options for select components
export const PRIORITY_OPTIONS = [
  { value: 'low', label: '低' },
  { value: 'medium', label: '中' },
  { value: 'high', label: '高' },
  { value: 'required', label: '必須' },
] as const;
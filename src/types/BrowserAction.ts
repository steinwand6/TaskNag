// Browser Action types for notification-triggered URL opening

export interface BrowserAction {
  id: string;
  label: string;
  url: string;
  enabled: boolean;
  order: number;
  createdAt: Date;
}

export interface BrowserActionSettings {
  enabled: boolean;
  actions: BrowserAction[];
}

export interface URLValidationResult {
  isValid: boolean;
  error?: string;
  suggestions?: string[];
}

export interface URLPreview {
  url: string;
  title: string;
  domain: string;
  faviconUrl?: string;
  description?: string;
}

// API Request interfaces
export interface CreateBrowserActionRequest {
  label: string;
  url: string;
  enabled?: boolean;
}

export interface UpdateBrowserActionRequest {
  label?: string;
  url?: string;
  enabled?: boolean;
  order?: number;
}

// Validation constraints
export const BROWSER_ACTION_CONSTRAINTS = {
  MAX_ACTIONS: 5,
  MAX_LABEL_LENGTH: 50,
  MAX_URL_LENGTH: 2000,
  MIN_LABEL_LENGTH: 1,
} as const;

// URL validation patterns
export const URL_PATTERNS = {
  DANGEROUS_PROTOCOLS: ['javascript:', 'data:', 'vbscript:', 'file:', 'about:'],
  ALLOWED_PROTOCOLS: ['http:', 'https:', 'ftp:', 'ftps:'],
  LOCAL_DOMAINS: ['localhost', '127.0.0.1', '0.0.0.0'],
} as const;

// Helper type for form validation
export interface BrowserActionFormData {
  label: string;
  url: string;
  enabled: boolean;
}

export interface BrowserActionFormErrors {
  label?: string;
  url?: string;
  general?: string;
}

// Drag and drop support
export interface DragDropItem {
  id: string;
  index: number;
}

// Component props interfaces
export interface URLInputProps {
  value: string;
  onChange: (value: string) => void;
  onValidation?: (result: URLValidationResult) => void;
  placeholder?: string;
  disabled?: boolean;
  showPreview?: boolean;
  showTestButton?: boolean;
  error?: string;
}

export interface URLActionConfigProps {
  actions: BrowserAction[];
  onChange: (actions: BrowserAction[]) => void;
  maxActions?: number;
  disabled?: boolean;
}

// State management interfaces
export interface BrowserActionState {
  actions: BrowserAction[];
  isValidating: boolean;
  validationResults: Record<string, URLValidationResult>;
  draggedItem: DragDropItem | null;
}
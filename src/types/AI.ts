export interface ModelInfo {
  name: string;
  modified_at: string;
  size: number;
}

export interface AIPersonality {
  id: string;
  name: string;
  description: string;
  tone_description: string;
  prompt_prefix: string;
  sample_phrases: string[];
  emoji_style: 'None' | 'Minimal' | 'Moderate' | 'Frequent';
}

export interface TaskAnalysis {
  improved_title: string;
  improved_description: string;
  suggested_tags: string[];
  complexity: 'simple' | 'medium' | 'complex';
  estimated_hours: number;
  subtasks: SubtaskSuggestion[];
  priority_reasoning: string;
}

export interface SubtaskSuggestion {
  title: string;
  description: string;
  order: number;
}

export interface ProjectPlan {
  phases: ProjectPhase[];
  total_estimated_days: number;
  dependencies: TaskDependency[];
  milestones: Milestone[];
}

export interface ProjectPhase {
  name: string;
  description: string;
  tasks: SubtaskSuggestion[];
  estimated_days: number;
  order: number;
}

export interface TaskDependency {
  from_task: string;
  to_task: string;
  dependency_type: 'blocks' | 'requires' | 'relates_to';
}

export interface Milestone {
  name: string;
  description: string;
  target_date?: string;
}

export interface AgentConfig {
  default_model: string;
  base_url: string;
  timeout_seconds: number;
  available_models: string[];
  model_preferences: Record<string, ModelPreference>;
}

export interface ModelPreference {
  display_name: string;
  description: string;
  recommended_for: string[];
  performance_tier: ModelPerformanceTier;
}

export enum ModelPerformanceTier {
  Fast = 'Fast',
  Balanced = 'Balanced',
  Quality = 'Quality',
}
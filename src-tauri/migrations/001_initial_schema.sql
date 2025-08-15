-- Initial schema for TaskNag

-- Tasks table with complete schema including notifications and browser actions
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK(status IN ('inbox', 'todo', 'in_progress', 'done')),
    parent_id TEXT,
    due_date TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    progress INTEGER DEFAULT 0 CHECK(progress >= 0 AND progress <= 100),
    
    -- Notification settings (replaces priority system)
    notification_type TEXT CHECK(notification_type IN ('none', 'due_date_based', 'recurring')),
    notification_days_before INTEGER,
    notification_time TEXT, -- HH:MM format
    notification_days_of_week TEXT, -- JSON array: "[0,1,2,3,4,5,6]" where 0=Sunday
    notification_level INTEGER CHECK(notification_level IN (1, 2, 3)),
    
    -- Browser actions for notifications (JSON)
    browser_actions TEXT, -- JSON: {"enabled": true, "actions": [...]}
    
    FOREIGN KEY (parent_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at TEXT NOT NULL
);

-- Task-Tags junction table
CREATE TABLE IF NOT EXISTS task_tags (
    task_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Ollama Agent Conversation Table
CREATE TABLE IF NOT EXISTS agent_conversations (
    id TEXT PRIMARY KEY,
    messages TEXT NOT NULL, -- JSON array of messages
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Agent Suggestions Table
CREATE TABLE IF NOT EXISTS agent_suggestions (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    suggestion_type TEXT NOT NULL, -- 'improvement', 'subtask', 'priority', etc.
    content TEXT NOT NULL, -- JSON content of suggestion
    applied BOOLEAN DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- Agent Configuration Table
CREATE TABLE IF NOT EXISTS agent_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id);
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
CREATE INDEX IF NOT EXISTS idx_tasks_notification_type ON tasks(notification_type);
CREATE INDEX IF NOT EXISTS idx_tasks_notification_level ON tasks(notification_level);
CREATE INDEX IF NOT EXISTS idx_tasks_browser_actions ON tasks(browser_actions) WHERE browser_actions IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_task_tags_task_id ON task_tags(task_id);
CREATE INDEX IF NOT EXISTS idx_task_tags_tag_id ON task_tags(tag_id);

-- Agent table indexes for performance
CREATE INDEX IF NOT EXISTS idx_suggestions_task_id ON agent_suggestions(task_id);
CREATE INDEX IF NOT EXISTS idx_suggestions_created_at ON agent_suggestions(created_at);
CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON agent_conversations(updated_at);

-- Insert default agent configuration
INSERT OR IGNORE INTO agent_config (key, value, updated_at) VALUES 
    ('ollama_base_url', 'http://localhost:11434', datetime('now')),
    ('default_model', 'llama3:latest', datetime('now')),
    ('timeout_seconds', '30', datetime('now')),
    ('auto_suggestions_enabled', 'false', datetime('now')),
    ('streaming_enabled', 'true', datetime('now'));
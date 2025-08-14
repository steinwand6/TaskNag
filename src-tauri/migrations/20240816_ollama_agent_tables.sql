-- Ollama Agent Conversation Table
CREATE TABLE IF NOT EXISTS agent_conversations (
    id TEXT PRIMARY KEY,
    messages TEXT NOT NULL, -- JSON array of messages
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Agent Suggestions Table
CREATE TABLE IF NOT EXISTS agent_suggestions (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    suggestion_type TEXT NOT NULL, -- 'improvement', 'subtask', 'priority', etc.
    content TEXT NOT NULL, -- JSON content of suggestion
    applied BOOLEAN DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- Agent Configuration Table
CREATE TABLE IF NOT EXISTS agent_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Insert default configuration
INSERT OR IGNORE INTO agent_config (key, value, updated_at) VALUES 
    ('ollama_base_url', 'http://localhost:11434', datetime('now')),
    ('default_model', 'llama3:latest', datetime('now')),
    ('timeout_seconds', '30', datetime('now')),
    ('auto_suggestions_enabled', 'false', datetime('now')),
    ('streaming_enabled', 'true', datetime('now'));

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_suggestions_task_id ON agent_suggestions(task_id);
CREATE INDEX IF NOT EXISTS idx_suggestions_created_at ON agent_suggestions(created_at);
CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON agent_conversations(updated_at);
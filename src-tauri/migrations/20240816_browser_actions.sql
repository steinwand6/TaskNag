-- Browser Actions for Task Notifications
-- Adds browser_actions column to tasks table for storing URL configurations

-- Add browser_actions column to tasks table
ALTER TABLE tasks ADD COLUMN browser_actions TEXT DEFAULT NULL;

-- Create index for better performance when querying tasks with browser actions
CREATE INDEX IF NOT EXISTS idx_tasks_browser_actions ON tasks(browser_actions) WHERE browser_actions IS NOT NULL;

-- The browser_actions column will store JSON data with the following structure:
-- {
--   "enabled": true,
--   "actions": [
--     {
--       "id": "uuid-v4-string",
--       "label": "会議室予約",
--       "url": "https://calendar.google.com/calendar/u/0/r",
--       "enabled": true,
--       "order": 1,
--       "created_at": "2024-08-16T12:00:00Z"
--     }
--   ]
-- }

-- Example queries:
-- SELECT * FROM tasks WHERE browser_actions IS NOT NULL;
-- SELECT * FROM tasks WHERE json_extract(browser_actions, '$.enabled') = 1;
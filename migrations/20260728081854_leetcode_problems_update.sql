-- Add migration script here

ALTER TABLE leetcode_problems DROP COLUMN category;

ALTER TABLE leetcode_problems ADD COLUMN frontend_id TEXT;
ALTER TABLE leetcode_problems ADD COLUMN topic_tag TEXT;

ALTER TABLE leetcode_problems ADD COLUMN level TEXT CHECK(level IN ('Easy', 'Medium', 'Hard'));

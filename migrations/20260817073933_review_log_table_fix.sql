-- Add migration script here

ALTER TABLE review_log RENAME TO review_logs;

ALTER TABLE review_logs RENAME COLUMN problme_id TO problem_id;

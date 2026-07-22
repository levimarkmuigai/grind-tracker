-- Add migration script here
ALTER TABLE fsrs_cards ADD COLUMN last_sync_at DATETIME;

-- Add migration script here

ALTER TABLE fsrs_cards RENAME TO fsrs_cards_old;

create TABLE fsrs_cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  problem_id INTEGER NOT NULL UNIQUE,

  stability REAL NOT NULL DEFAULT 0.0,
  difficulty REAL NOT NULL DEFAULT 0.0,

  due_date DATETIME DEFAULT NULL,
  state INTEGER NOT NULL DEFAULT 0,
  reps INTEGER NOT NULL DEFAULT 0,
  lapses INTEGER NOT NULL DEFAULT 0,
  last_sync_at DATETIME NULL,

  FOREIGN KEY(problem_id) REFERENCES leetcode_problems(id) ON DELETE CASCADE
);

INSERT INTO fsrs_cards SELECT * FROM fsrs_cards_old;

UPDATE fsrs_cards SET due_date = NULL WHERE state = 0 AND due_date IS NOT NULL;

DROP TABLE fsrs_cards_old;

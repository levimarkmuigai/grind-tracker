-- Add migration script here

CREATE TABLE leetcode_problems (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  category TEXT NOT NULL
);

create TABLE fsrs_cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  problem_id INTEGER NOT NULL UNIQUE,

  stability REAL NOT NULL DEFAULT 0.0,
  difficulty REAL NOT NULL DEFAULT 0.0,

  due_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  state INTEGER NOT NULL DEFAULT 0, -- 0: New, 1: Learning, 2: Review, 3: Relearning
  reps INTEGER NOT NULL DEFAULT 0,
  lapses INTEGER NOT NULL DEFAULT 0,

  FOREIGN KEY(problem_id) REFERENCES leetcode_problems(id) ON DELETE CASCADE
);

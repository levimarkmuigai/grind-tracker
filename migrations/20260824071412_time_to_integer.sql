-- Add migration script here

ALTER TABLE fsrs_cards RENAME TO fsrs_cards_old;

CREATE TABLE fsrs_cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  problem_id INTEGER NOT NULL UNIQUE,

  stability REAL NOT NULL DEFAULT 0.0,
  difficulty REAL NOT NULL DEFAULT 0.0,

  due_date INTEGER,
  state INTEGER NOT NULL DEFAULT 0,
  reps INTEGER NOT NULL DEFAULT 0,
  lapses INTEGER NOT NULL DEFAULT 0,
  last_sync_at INTEGER,

  FOREIGN KEY(problem_id) REFERENCES leetcode_problems(id) ON DELETE CASCADE
);

INSERT INTO fsrs_cards SELECT id, problem_id, stability, difficulty,
CASE
WHEN typeof(due_date) = 'integer' THEN due_date
ELSE unixepoch(due_date)
END, state, reps, lapses,
CASE
WHEN last_sync_at IS NULL THEN NULL
WHEN typeof(last_sync_at) = 'integer' THEN last_sync_at
ELSE unixepoch(last_sync_at)
END
FROM fsrs_cards_old;

UPDATE fsrs_cards SET due_date = NULL WHERE state = 0 AND due_date IS NOT NULL;

DROP TABLE fsrs_cards_old;

DROP INDEX IF EXISTS idx_review_logs_reviewed_at;
DROP INDEX IF EXISTS idx_review_logs_problem_id;

ALTER TABLE review_logs RENAME TO review_logs_old;

CREATE TABLE review_logs (
    id          INTEGER PRIMARY KEY,
    problem_id  INTEGER NOT NULL,
    reviewed_at INTEGER NOT NULL DEFAULT (unixepoch()),
    rating      INTEGER NOT NULL CHECK (rating IN (1, 2, 3, 4)),

    FOREIGN KEY (problem_id)
        REFERENCES leetcode_problems(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_review_logs_reviewed_at ON review_logs (reviewed_at);
CREATE INDEX IF NOT EXISTS idx_review_logs_problem_id  ON review_logs (problem_id);

INSERT INTO review_logs (id, problem_id, reviewed_at, rating)
SELECT id, problem_id, CASE
WHEN typeof(reviewed_at) = 'integer' THEN reviewed_at
ELSE unixepoch(reviewed_at)
END, rating
FROM review_logs_old;

DROP TABLE review_logs_old;


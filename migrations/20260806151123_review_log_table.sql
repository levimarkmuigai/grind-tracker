-- Add migration script here
CREATE TABLE review_log (
  id  INTEGER PRIMARY KEY,
  problme_id INTEGER NOT NULL REFERENCES leetcode_problems(id) ON DELETE CASCADE,
  reviewed_at DATETIME NOT NULL DEFAULT (datetime('now')),
  rating INTEGER NOT NULL CHECK(rating IN (1,2,3,4))
);

CREATE INDEX idx_review_log_date ON review_log (DATE(reviewed_at));
CREATE INDEX idx_review_log_problem ON review_log (problme_id);

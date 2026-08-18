ALTER TABLE review_logs RENAME TO review_logs_old;

CREATE TABLE review_logs (
    id          INTEGER PRIMARY KEY,
    problem_id  INTEGER NOT NULL,
    reviewed_at DATETIME NOT NULL DEFAULT (datetime('now')),
    rating      INTEGER NOT NULL CHECK (rating IN (1, 2, 3, 4)),

    FOREIGN KEY (problem_id)
        REFERENCES leetcode_problems(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_review_logs_reviewed_at ON review_logs (reviewed_at);
CREATE INDEX idx_review_logs_problem_id  ON review_logs (problem_id);

INSERT INTO review_logs (id, problem_id, reviewed_at, rating)
SELECT id, problem_id, reviewed_at, rating
FROM review_logs_old;

DROP TABLE review_logs_old;

-- Progress: which course modules a student has completed
CREATE TABLE IF NOT EXISTS module_completions (
    user_id TEXT NOT NULL,
    course_module_id TEXT NOT NULL,
    completed_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (user_id, course_module_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (course_module_id) REFERENCES course_modules(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_module_completions_user_id ON module_completions(user_id);
CREATE INDEX IF NOT EXISTS idx_module_completions_module_id ON module_completions(course_module_id);

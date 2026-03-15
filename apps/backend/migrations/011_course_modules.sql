-- Course curriculum / syllabus: modules per course
CREATE TABLE IF NOT EXISTS course_modules (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL,
    title TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    link TEXT,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_course_modules_course_id ON course_modules(course_id);

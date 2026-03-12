-- Role column: student (default), admin, super_admin
ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'student';

-- Invites: token, role, optional email, expiry, used tracking
CREATE TABLE IF NOT EXISTS invites (
    id TEXT PRIMARY KEY NOT NULL,
    token TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL,
    email TEXT,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    used_at TEXT,
    used_by TEXT,
    FOREIGN KEY (used_by) REFERENCES users(id)
);
CREATE INDEX IF NOT EXISTS idx_invites_token ON invites(token);
CREATE INDEX IF NOT EXISTS idx_invites_expires_at ON invites(expires_at);

-- Courses: optional fields for admin
ALTER TABLE courses ADD COLUMN description TEXT;
ALTER TABLE courses ADD COLUMN price REAL DEFAULT 0;
ALTER TABLE courses ADD COLUMN active INTEGER DEFAULT 1;

-- Learning plans (admin-managed)
CREATE TABLE IF NOT EXISTS learning_plans (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    price REAL NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'NGN',
    duration TEXT NOT NULL DEFAULT '',
    course_ids TEXT NOT NULL DEFAULT '[]',
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

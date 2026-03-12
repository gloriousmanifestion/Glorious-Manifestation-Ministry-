-- Users table for Pastoral School registration
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    ministry_role TEXT NOT NULL,
    church_name TEXT NOT NULL,
    calling TEXT NOT NULL,
    selected_course_id TEXT,
    verified INTEGER NOT NULL DEFAULT 0,
    payment_complete INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Courses (reference data, can be seeded)
CREATE TABLE IF NOT EXISTS courses (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    duration TEXT NOT NULL
);

INSERT INTO courses (id, name, duration) VALUES
    ('pastoral-leadership', 'Pastoral Leadership', '6 months'),
    ('prophetic-ministry', 'Prophetic Ministry', '6 months'),
    ('ministerial-training', 'Ministerial Training', '6 months'),
    ('biblical-studies', 'Biblical Studies', '6 months'),
    ('worship-praise', 'Worship & Praise', '6 months'),
    ('evangelism-outreach', 'Evangelism & Outreach', '6 months')
ON CONFLICT(id) DO NOTHING;

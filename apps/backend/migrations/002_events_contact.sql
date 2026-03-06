-- Events table for Pastoral School calendar
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    date TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Seed default events
INSERT OR IGNORE INTO events (id, title, date, description) VALUES
    ('school-opening', 'School Opening', 'February', 'Pastoral School session begins. Orientation and registration.'),
    ('mid-term-revival', 'Mid-term Revival', 'April', 'Special revival and impartation service for students.'),
    ('graduation', 'Graduation Ceremony', 'July', 'Graduation and commissioning of new ministers.'),
    ('weekly-chapel', 'Weekly Chapel', 'Every Friday', 'Corporate worship and teaching sessions.');

-- Contact form submissions
CREATE TABLE IF NOT EXISTS contact_submissions (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    subject TEXT,
    message TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

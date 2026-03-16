-- Church services: admin-managed weekly services (displayed on /church).
CREATE TABLE IF NOT EXISTS church_services (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    day_of_week TEXT NOT NULL,
    time TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    image_url TEXT,
    features TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    updated_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS idx_church_services_active_order ON church_services(is_active, sort_order);

-- Site content: key-value for editable page sections (church about, hero text, etc.).
CREATE TABLE IF NOT EXISTS site_content (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

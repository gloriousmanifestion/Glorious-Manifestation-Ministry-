-- Testimonials: quotes from students/alumni (admin-managed, shown on site).
CREATE TABLE IF NOT EXISTS testimonials (
    id TEXT PRIMARY KEY NOT NULL,
    author_name TEXT NOT NULL,
    role_or_title TEXT,
    body TEXT NOT NULL,
    image_url TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS idx_testimonials_sort ON testimonials(sort_order, created_at DESC);

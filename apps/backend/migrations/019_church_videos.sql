-- Ministry videos for church gallery: link (url) or uploaded file (file_path).
-- At least one of url or file_path should be set.
CREATE TABLE IF NOT EXISTS church_videos (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    url TEXT,
    file_path TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS idx_church_videos_sort ON church_videos(sort_order);

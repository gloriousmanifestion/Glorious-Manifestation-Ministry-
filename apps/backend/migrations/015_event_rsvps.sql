-- Event RSVPs: one row per user per event
CREATE TABLE IF NOT EXISTS event_rsvps (
    event_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (event_id, user_id),
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

-- SQLite uses different syntax for default timestamp
-- PostgreSQL: use DEFAULT (CURRENT_TIMESTAMP) or similar if needed at runtime
CREATE INDEX IF NOT EXISTS idx_event_rsvps_event_id ON event_rsvps(event_id);
CREATE INDEX IF NOT EXISTS idx_event_rsvps_user_id ON event_rsvps(user_id);

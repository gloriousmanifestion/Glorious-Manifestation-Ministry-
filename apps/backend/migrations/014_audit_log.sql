-- Audit log for admin actions (who did what, when)
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    admin_user_id TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT,
    target_id TEXT,
    details TEXT,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    -- admin_user_id stores who performed the action; no FK so log survives user deletion
);

CREATE INDEX IF NOT EXISTS idx_audit_log_admin_user_id ON audit_log(admin_user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_created_at ON audit_log(created_at DESC);

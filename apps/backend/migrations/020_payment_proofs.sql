-- Create payment_proofs table for proof-of-payment uploads.
-- This table backs:
-- - POST /api/v1/payments/proofs
-- - GET  /api/v1/payments/proofs/me

-- Postgres + SQLite compatible schema (sqlx::any).
CREATE TABLE IF NOT EXISTS payment_proofs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    file_url TEXT NOT NULL,
    status TEXT NOT NULL,
    note TEXT NULL,
    uploaded_at TEXT NOT NULL,
    reviewed_at TEXT NULL,
    reviewed_by TEXT NULL,
    comment TEXT NULL
);


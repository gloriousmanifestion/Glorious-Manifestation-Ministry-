-- Email verification tokens for account verification flow.
-- Similar to password_reset_tokens but dedicated to verifying email/marking users.verified = 1.

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL,
    used_at TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_email_verif_token ON email_verification_tokens(token);
CREATE INDEX IF NOT EXISTS idx_email_verif_user_id ON email_verification_tokens(user_id);


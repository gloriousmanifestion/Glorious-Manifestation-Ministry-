-- Optional TOTP secret for admin 2FA (nullable; when set, admin must provide code at login).
-- Full 2FA flow (enable/verify TOTP, login step) can be implemented when needed.
ALTER TABLE users ADD COLUMN totp_secret TEXT;

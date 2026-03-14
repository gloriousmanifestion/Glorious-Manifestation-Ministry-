-- Link courses to Microsoft Teams classes (education class or team id).
-- Used to show "Open in Teams" on the student dashboard.
ALTER TABLE courses ADD COLUMN teams_class_id TEXT;

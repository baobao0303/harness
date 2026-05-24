-- Harness v0 schema migration 002
-- Add test_skill TEXT column to story table.

ALTER TABLE story ADD COLUMN test_skill TEXT;

INSERT INTO schema_version (version) VALUES (2);

-- Harness v0 schema - migration 005
-- Add task priority classification (P0, P1, P2, P3) to story and backlog tables.

ALTER TABLE story ADD COLUMN priority TEXT NOT NULL DEFAULT 'P2' CHECK(priority IN ('P0','P1','P2','P3'));
ALTER TABLE backlog ADD COLUMN priority TEXT NOT NULL DEFAULT 'P2' CHECK(priority IN ('P0','P1','P2','P3'));

INSERT INTO schema_version (version) VALUES (5);

-- Harness v0 schema - migration 007
-- Add harness_experience table for self-learning library (Harness2 pattern)

CREATE TABLE IF NOT EXISTS harness_experience (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    domain               TEXT    NOT NULL,
    work_item_id         INTEGER REFERENCES work_item(id) ON DELETE SET NULL,
    cue                  TEXT    NOT NULL,
    lesson               TEXT    NOT NULL,
    evidence             TEXT    NOT NULL,
    confidence           TEXT    NOT NULL DEFAULT 'medium'
                         CHECK(confidence IN ('low', 'medium', 'high')),
    harness_patch        TEXT,
    created_at           TEXT    NOT NULL DEFAULT (datetime('now')),
    pattern_hash         TEXT,
    successful_harness   TEXT,
    failure_trajectories TEXT
);

CREATE INDEX IF NOT EXISTS idx_harness_experience_domain ON harness_experience(domain);
CREATE INDEX IF NOT EXISTS idx_harness_experience_work_item ON harness_experience(work_item_id);
CREATE INDEX IF NOT EXISTS idx_exp_domain_pattern ON harness_experience(domain, pattern_hash);

INSERT INTO schema_version (version) VALUES (7);

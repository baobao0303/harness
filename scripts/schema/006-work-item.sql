-- Harness v0 schema - migration 006
-- Add unified work_item table supporting Epic, Feature, User Story, Technical Story, Task, Bug, Testcase
-- Complies with spec_new.md Section 5.2-5.4 & 6.4

CREATE TABLE IF NOT EXISTS work_item (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    type                 TEXT    NOT NULL
                         CHECK(type IN (
                             'Epic', 'Feature', 'User Story', 'Technical Story', 'Task', 'Bug', 'Testcase'
                         )),
    title                TEXT    NOT NULL,
    description          TEXT,
    state                TEXT    NOT NULL DEFAULT 'New'
                         CHECK(state IN (
                             'New', 'Accepted', 'Active', 'Resolved', 'Closed', 'Blocked', 'Removed'
                         )),
    assigned_to          TEXT,
    story_points         INTEGER CHECK(story_points IS NULL OR story_points >= 0),
    remaining_work       REAL    CHECK(remaining_work IS NULL OR remaining_work >= 0),
    priority             TEXT    NOT NULL DEFAULT 'P2'
                         CHECK(priority IN ('P0', 'P1', 'P2', 'P3')),
    severity             TEXT    CHECK(severity IS NULL OR severity IN ('Low', 'Medium', 'High', 'Critical')),
    parent_id            INTEGER REFERENCES work_item(id) ON DELETE SET NULL,
    area_path            TEXT,
    iteration_path       TEXT,
    tags                 TEXT,
    acceptance_criteria  TEXT,
    repro_steps          TEXT,
    actual_result        TEXT,
    expected_result      TEXT,
    steps                TEXT,
    created_at           TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at           TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_work_item_type ON work_item(type);
CREATE INDEX IF NOT EXISTS idx_work_item_state ON work_item(state);
CREATE INDEX IF NOT EXISTS idx_work_item_parent ON work_item(parent_id);
CREATE INDEX IF NOT EXISTS idx_work_item_assigned ON work_item(assigned_to);

INSERT INTO schema_version (version) VALUES (6);

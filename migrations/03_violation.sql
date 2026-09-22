CREATE TABLE IF NOT EXISTS violations (
    driver_id           BLOB NOT NULL,
    id                  BLOB NOT NULL,
    violation_type      TEXT NOT NULL,
    base_fine_range     REAL NOT NULL,
    has_warning         INTEGER NOT NULL,
    ban_duration_months INTEGER NOT NULL,
    created_at          TEXT NOT NULL,
    updated_at          TEXT,
    PRIMARY KEY         (driber_id, id)
);

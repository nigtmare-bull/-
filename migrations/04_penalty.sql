CREATE TABLE IF NOT EXISTS penalties (

    id                    BLOB  NOT NULL,
    violation_id          BLOB NOT NULL,
    datetime              TEXT NOT NULL,
    driver_id             BLOB  NOT NULL,
    district              TEXT NOT NULL,
    fine_amount           REAL NOT NULL,
    is_paid               INTEGER NOT NULL,
    suspension_months     INTEGER NOT NULL,
    officer_id            TEXT NOT NULL,
    created_at            TEXT NOT NULL,
    updated_at            TEXT,
    CONSTRAINT fk_violation FOREIGN KEY (violation_id) REFERENCES violations(id) ON DELETE RESTRICT,
    CONSTRAINT fk_driver FOREIGN KEY (driver_license_number) REFERENCES drivers(license_number) ON DELETE CASCADE
    PRIMARY KEY (driver_id, id, violation_id,)
);

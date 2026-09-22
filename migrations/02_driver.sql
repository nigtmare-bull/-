CREATE TABLE IF NOT EXISTS drivers (
    license_number   TEXT PRIMARY KEY NOT NULL,
    full_name        TEXT NOT NULL,
    address          TEXT NOT NULL,
    phone_number     TEXT NOT NULL,
    created_at       TEXT NOT NULL,
    updated_at       TEXT,
    deleted_at       TEXT
);



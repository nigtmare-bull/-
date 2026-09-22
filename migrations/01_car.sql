CREATE TABLE IF NOT EXISTS cars (
    id                  BLOB PRIMARY KEY NOT NULL
    car_number          TEXT NOT NULL,
    brand               TEXT NOT NULL,
    model               TEXT NOT NULL,
    color               TEXT NOT NULL,
    year_of_manufacture INTEGER NOT NULL,
    registration_date   TEXT NOT NULL,
    created_at          TEXT NOT NULL,
    updated_at          TEXT,
    deleted_at          TEXT

);
CREATE UNIQUE INDEX IF NOT EXISTS idx_car_number ON cars (car_number);

CREATE TABLE parking_info (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    parking_type VARCHAR(50) NOT NULL,
    total_capacity SMALLINT NOT NULL,
    short_capacity SMALLINT NOT NULL,
    other_capacity SMALLINT NOT NULL,
    updated_time TIMESTAMP DEFAULT NOW() NOT NULL
)

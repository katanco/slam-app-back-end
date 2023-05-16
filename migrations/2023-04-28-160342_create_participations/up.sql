-- Your SQL goes here
CREATE TABLE participations (
    id TEXT PRIMARY KEY NOT NULL,
    performance_notes TEXT,
    performance_length_in_seconds INTEGER,
    deduction REAL,
    score REAL,
    performance_order INTEGER NOT NULL,
    round_id TEXT REFERENCES rounds(id) ON DELETE CASCADE NOT NULL,
    participant_id TEXT REFERENCES participants(id) ON DELETE CASCADE NOT NULL
);  
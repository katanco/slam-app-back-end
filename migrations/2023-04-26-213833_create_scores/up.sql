CREATE TABLE scores (
    id TEXT PRIMARY KEY NOT NULL,
    value REAL NOT NULL,
    participation_id TEXT REFERENCES participation(id) NOT NULL,
    submitter_id TEXT
);
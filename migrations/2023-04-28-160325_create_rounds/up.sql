-- Your SQL goes here
CREATE TABLE rounds (
    id TEXT PRIMARY KEY NOT NULL,
    round_number INTEGER NOT NULL,
    room_id TEXT REFERENCES rooms(id) ON DELETE CASCADE NOT NULL
);
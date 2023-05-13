CREATE TABLE participants (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    pronouns TEXT,
    room_id TEXT REFERENCES rooms(id) NOT NULL
);
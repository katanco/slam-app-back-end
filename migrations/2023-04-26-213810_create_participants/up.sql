CREATE TABLE participants (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    pronouns TEXT,
    room_id TEXT REFERENCES rooms(id) ON DELETE CASCADE NOT NULL
);
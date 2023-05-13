CREATE TABLE rooms (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created TEXT NOT NULL,
    round_id_current TEXT REFERENCES rounds(id),
    participation_id_current TEXT REFERENCES participations(id)
);
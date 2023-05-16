-- Your SQL goes here
ALTER TABLE
    scores
ADD
    participation_id TEXT REFERENCES participations(id) ON DELETE CASCADE NOT NULL;

ALTER TABLE
    rooms
ADD
    round_id_current TEXT REFERENCES rounds(id),
ADD
    participation_id_current TEXT REFERENCES participations(id);
-- This file should undo anything in `up.sql`
ALTER TABLE
    scores
REMOVE
    participation_id;

ALTER TABLE
    rooms
REMOVE
    round_id_current,
REMOVE
    participation_id_current;
use chrono::{ DateTime, Utc };
use diesel::{ prelude::*, pg::PgConnection };
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
use std::{ time::SystemTime };
use uuid::Uuid;
use crate::models::*;
use dotenv::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration() {
    establish_connection().run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn insert_room(conn: &mut PgConnection, name_value: &str) -> Room {
    use crate::schema::rooms::dsl::*;
    let new_room = Room {
        id: Uuid::new_v4().to_string(),
        name: name_value.to_owned(),
        created: iso_date(),
        round_id_current: None,
        participation_id_current: None,
    };
    diesel::insert_into(rooms).values(&new_room).execute(conn).expect("Error inserting room");
    return new_room;
}

pub fn insert_participant(
    conn: &mut PgConnection,
    name_value: &str,
    pronouns_value: Option<String>,
    room_id_value: &str
) -> Participant {
    use crate::schema::participants::dsl::*;
    let new_participant = Participant {
        id: Uuid::new_v4().to_string(),
        name: name_value.to_owned(),
        pronouns: pronouns_value,
        room_id: room_id_value.to_string(),
    };
    let existing_participant: Option<Participant> = participants
        .filter(name.eq(name_value))
        .first(conn)
        .optional()
        .unwrap();
    if existing_participant.is_some() {
        panic!("Existing participant");
    }
    {
        use crate::schema::rooms::dsl::*;
        let matching_room: Option<Room> = rooms
            .filter(id.eq(room_id_value))
            .first(conn)
            .optional()
            .unwrap();
        if matching_room.is_none() {
            panic!("No matching room id");
        }
    }

    diesel
        ::insert_into(participants)
        .values(&new_participant)
        .execute(conn)
        .expect("Error inserting participant");
    return new_participant;
}

pub fn update_room(conn: &mut PgConnection, id_value: String, name_value: Option<String>) -> usize {
    use crate::schema::rooms::dsl::*;
    let result = diesel
        ::update(rooms.filter(id.eq(id_value)))
        .set(
            &(RoomUpdate {
                name: name_value,
            })
        )
        .execute(conn)
        .expect("unable to update room");
    return result;
}

pub fn update_participant(
    conn: &mut PgConnection,
    id_value: String,
    name_value: Option<String>,
    pronouns_value: Option<String>
) -> usize {
    use crate::schema::participants::dsl::*;
    let result = diesel
        ::update(participants.filter(id.eq(id_value)))
        .set(
            &(ParticipantUpdate {
                name: name_value,
                pronouns: pronouns_value,
            })
        )
        .execute(conn)
        .expect("unable to update participant");
    return result;
}

pub fn update_participation(
    conn: &mut PgConnection,
    id_value: String,
    notes_value: Option<String>,
    length_value: Option<i32>
) -> usize {
    use crate::schema::participations::dsl::*;
    let mut deduction_value = None;
    if let Some(length) = length_value {
        if length > 190 {
            deduction_value = Some((((length as f32) - 190_f32) / 10_f32).floor() * 0.5);
        }
    }

    let result = diesel
        ::update(participations.filter(id.eq(id_value)))
        .set(
            &(ParticipationUpdate {
                performance_length_in_seconds: length_value,
                performance_notes: notes_value,
                deduction: deduction_value,
                score: None,
            })
        )
        .execute(conn)
        .expect("unable to update participation");
    return result;
}

pub fn remove_room(conn: &mut PgConnection, id_value: String) -> usize {
    use crate::schema::rooms::dsl::*;
    let id_value_clone = id_value.clone();

    let result = diesel
        ::delete(rooms.filter(id.eq(id_value)))
        .execute(conn)
        .expect("unable to delete room");
    {
        use crate::schema::participants::dsl::*;
        diesel
            ::delete(participants.filter(room_id.eq(id_value_clone)))
            .execute(conn)
            .expect("unable to delete participants");
    }
    return result;
}

pub fn remove_participant(conn: &mut PgConnection, id_value: String) -> usize {
    use crate::schema::participants::dsl::*;
    let result = diesel
        ::delete(participants.filter(id.eq(id_value)))
        .execute(conn)
        .expect("unable to delete participant");
    return result;
}

pub fn insert_score(
    conn: &mut PgConnection,
    value_value: &f32,
    participation_id_value: &str
) -> Score {
    use crate::schema::scores::dsl::*;
    let new_score = Score {
        id: Uuid::new_v4().to_string(),
        value: value_value.to_owned(),
        submitter_id: None,
        participation_id: participation_id_value.to_string(),
    };
    diesel::insert_into(scores).values(&new_score).execute(conn).expect("Error inserting score");

    let _scores: Vec<Score> = scores
        .filter(participation_id.eq(participation_id_value))
        .order(value.asc())
        .load::<Score>(conn)
        .expect("Error aggregating scores");

    if _scores.len() == 5 {
        let mut aggregate_score = 0_f32;
        for (pos, score_iter) in _scores.iter().enumerate() {
            if pos != 0 && pos != 4 {
                aggregate_score += score_iter.value;
            }
        }

        use crate::schema::participations::dsl::*;
        diesel
            ::update(participations.filter(id.eq(participation_id_value)))
            .set(
                &(ParticipationUpdate {
                    performance_length_in_seconds: None,
                    performance_notes: None,
                    deduction: None,
                    score: Some(aggregate_score),
                })
            )
            .execute(conn)
            .expect("unable to update participation");
    }

    return new_score;
}

pub fn retrieve_rooms(conn: &mut PgConnection) -> Vec<Room> {
    use crate::schema::rooms::dsl::*;
    let results = rooms
        .order(created.desc())
        .limit(10)
        .load::<Room>(conn)
        .expect("Error loading rooms");

    return results;
}

pub fn retrieve_room(conn: &mut PgConnection, room_id_parameter: &str) -> RoomResponse {
    use crate::schema::rooms::dsl::*;
    let room_results: Room = rooms.find(room_id_parameter).first(conn).expect("Error loading room");

    use crate::schema::participants::dsl::*;
    let match_value = room_id_parameter.to_owned();
    let participant_results = participants
        .filter(crate::schema::participants::dsl::room_id.eq(match_value.clone()))
        .load::<Participant>(conn)
        .expect("Error loading participants");

    use crate::schema::rounds::dsl::*;
    let round_results = rounds
        .filter(crate::schema::rounds::dsl::room_id.eq(match_value))
        .load::<Round>(conn)
        .expect("Error loading rounds");

    let results = RoomResponse {
        room: room_results,
        participants: participant_results,
        rounds: round_results,
    };

    return results;
}

pub fn retrieve_round(conn: &mut PgConnection, round_id_parameter: &str) -> RoundResponse {
    use crate::schema::rounds::dsl::*;
    let round_results: Round = rounds
        .find(round_id_parameter)
        .first(conn)
        .expect("Error loading round");

    use crate::schema::participants::dsl::*;

    let participation_results: Vec<(Participation, Participant)> = Participation::belonging_to(
        &round_results
    )
        .inner_join(participants)
        .select((Participation::as_select(), Participant::as_select()))
        .load::<(Participation, Participant)>(conn)
        .expect("Error loading participations");

    let transformed_participation_results = participation_results
        .into_iter()
        .map(|(participation, participant)| ParticipationResponse { participation, participant })
        .collect();

    let results = RoundResponse {
        round: round_results,
        participations: transformed_participation_results,
    };

    return results;
}

pub fn create_next_round(
    conn: &mut PgConnection,
    room_id_parameter: &str,
    participants: Vec<Participant>
) -> Round {
    use crate::schema::rounds::dsl::*;

    let previous_round_response: Option<Round> = rounds
        .filter(room_id.eq(room_id_parameter))
        .first(conn)
        .optional()
        .unwrap();

    let mut new_round_number = 1;

    if let Some(previous_round) = previous_round_response {
        let prev_num = previous_round.round_number;
        new_round_number += prev_num;
    }

    let new_round = Round {
        id: Uuid::new_v4().to_string(),
        room_id: room_id_parameter.to_owned(),
        round_number: new_round_number,
    };
    diesel::insert_into(rounds).values(&new_round).execute(conn).expect("Error inserting round");

    use crate::schema::rooms::dsl::*;
    diesel
        ::update(rooms)
        .filter(crate::schema::rooms::id.eq(room_id_parameter))
        .set(round_id_current.eq(&new_round.id))
        .execute(conn)
        .expect("Error updating room");

    let mut vec: Vec<Participation> = Vec::new();
    let parameter_round_id = &new_round.id;

    for (pos, participant) in participants.iter().enumerate() {
        let parameter_participant_id = &participant.id;

        vec.push(Participation {
            id: Uuid::new_v4().to_string(),
            participant_id: parameter_participant_id.to_string(),
            performance_order: pos as i32,
            round_id: parameter_round_id.to_string(),
            deduction: None,
            performance_length_in_seconds: None,
            performance_notes: None,
            score: None,
        });
    }

    use crate::schema::participations::dsl::*;

    diesel
        ::insert_into(participations)
        .values(vec)
        .execute(conn)
        .expect("Error inserting participations");

    return new_round;
}

pub fn retrieve_participants(
    conn: &mut PgConnection,
    room_id_parameter: &Option<String>
) -> Vec<Participant> {
    use crate::schema::participants::dsl::*;

    let mut query = participants.into_boxed();

    if let Some(room_id_parameter) = room_id_parameter {
        query = query.filter(room_id.eq(room_id_parameter));
    }

    let results = query.load::<Participant>(conn).expect("Error loading participants");

    return results;
}

pub fn retrieve_scores(
    conn: &mut PgConnection,
    participation_id_parameter: &Option<String>
) -> Vec<Score> {
    use crate::schema::scores::dsl::*;

    let mut query = scores.into_boxed();

    if let Some(participation_id_parameter) = participation_id_parameter {
        query = query.filter(participation_id.eq(participation_id_parameter));
    }

    let results = query.load::<Score>(conn).expect("Error loading scores");

    return results;
}

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}
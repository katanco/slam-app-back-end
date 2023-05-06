use chrono::{ DateTime, Utc };
use diesel::{ prelude::* };
use std::{ time::SystemTime };
use uuid::Uuid;
use crate::models::{ Room, Participant, Score, RoomResponse, ParticipantUpdate };
use dotenv::dotenv;
use std::env;

pub fn insert_room(conn: &mut SqliteConnection, name_value: &str) -> Room {
    use crate::schema::rooms::dsl::*;
    let new_room = Room {
        id: Uuid::new_v4().to_string(),
        name: name_value.to_owned(),
        created: iso_date(),
    };
    diesel::insert_into(rooms).values(&new_room).execute(conn).expect("Error inserting room");
    return new_room;
}

pub fn insert_participant(
    conn: &mut SqliteConnection,
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
    diesel
        ::insert_into(participants)
        .values(&new_participant)
        .execute(conn)
        .expect("Error inserting participant");
    return new_participant;
}

pub fn update_participant(
    conn: &mut SqliteConnection,
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

pub fn insert_score(
    conn: &mut SqliteConnection,
    value_value: &f32,
    participation_id_value: &str
) -> Score {
    use crate::schema::scores::dsl::*;
    let new_score = Score {
        id: Uuid::new_v4().to_string(),
        value: value_value.to_owned(),
        participation_id: participation_id_value.to_string(),
    };
    diesel::insert_into(scores).values(&new_score).execute(conn).expect("Error inserting score");
    return new_score;
}

pub fn retrieve_rooms(conn: &mut SqliteConnection) -> Vec<Room> {
    use crate::schema::rooms::dsl::*;
    let results = rooms.order(created.desc()).limit(10).load::<Room>(conn).expect("Error loading rooms");

    return results;
}

pub fn retrieve_room(conn: &mut SqliteConnection, room_id_parameter: &str) -> RoomResponse {
    use crate::schema::rooms::dsl::*;
    let room_results: Room = rooms.find(room_id_parameter).first(conn).expect("Error loading room");

    use crate::schema::participants::dsl::*;
    let match_value = room_id_parameter.to_owned();
    let participant_results = participants
        .filter(room_id.eq(match_value))
        .load::<Participant>(conn)
        .expect("Error loading participants");

    let results = RoomResponse { room: room_results, participants: participant_results };

    return results;
}

pub fn retrieve_participants(
    conn: &mut SqliteConnection,
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
    conn: &mut SqliteConnection,
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

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}
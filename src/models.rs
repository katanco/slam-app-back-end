use serde::{ Deserialize, Serialize };
use crate::schema::*;
use diesel::{ Insertable, Queryable, AsChangeset, Identifiable, Associations, Selectable };

// Tables

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub created: String,
    pub round_id_current: Option<String>,
    pub participation_id_current: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Room))]
#[diesel(table_name = participants)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub pronouns: Option<String>,
    pub room_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Participation))]
#[diesel(table_name = scores)]
pub struct Score {
    pub id: String,
    pub value: f32,
    pub submitter_id: Option<String>,
    pub participation_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Room))]
#[diesel(table_name = rounds)]
pub struct Round {
    pub id: String,
    pub round_number: i32,
    pub room_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Round))]
#[diesel(belongs_to(Participant))]
#[diesel(table_name = participations)]
pub struct Participation {
    pub id: String,
    pub performance_notes: Option<String>,
    pub performance_length_in_seconds: Option<i32>,
    pub deduction: Option<f32>,
    pub score: Option<f32>,
    pub performance_order: i32,
    pub round_id: String,
    pub participant_id: String,
}

// Requests

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomRequest {
    pub name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantRequest {
    pub name: Option<String>,
    pub pronouns: Option<String>,
    pub room_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub value: f32,
    pub participation_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationRequest {
    pub notes: Option<String>,
    pub length: Option<i32>,
}

// Updates
#[derive(AsChangeset)]
#[diesel(table_name = participants)]
pub struct ParticipantUpdate {
    pub name: Option<String>,
    pub pronouns: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = rooms)]
pub struct RoomUpdate {
    pub name: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = participations)]
pub struct ParticipationUpdate {
    pub performance_notes: Option<String>,
    pub performance_length_in_seconds: Option<i32>,
    pub deduction: Option<f32>,
    pub score: Option<f32>,
}



// Response

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub room: Room,
    pub participants: Vec<Participant>,
    pub rounds: Vec<Round>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResponse {
    pub round: Round,
    pub participations: Vec<ParticipationResponse>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationResponse {
    pub participation: Participation,
    pub participant: Participant
}

// Filter

#[derive(Serialize, Deserialize)]
pub struct ParticipantFilter {
    pub room_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ScoreFilter {
    pub participation_id: Option<String>,
}
use serde::{ Deserialize, Serialize };
use crate::schema::*;
use diesel::{ Insertable, Queryable, AsChangeset, Identifiable };

// Tables

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub created: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Identifiable)]
#[diesel(table_name = participants)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub pronouns: Option<String>,
    pub room_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = scores)]
pub struct Score {
    pub id: String,
    pub value: f32,
    pub participation_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = rounds)]
pub struct Round {
    pub id: String,
    pub round_number: i32,
    pub room_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = participations)]
pub struct Participation {
    pub id: String,
    pub performance_notes: Option<String>,
    pub performance_length_in_seconds: Option<i32>,
    pub deduction: Option<f32>,
    pub performance_order: i32,
    pub score: Option<f32>,
    pub round_id: String,
    pub participant_id: String,
}

// Requests

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomRequest {
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pronouns: Option<String>,
    pub room_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub value: f32,
    pub participant_id: String,
}

// Updates
#[derive(AsChangeset)]
#[diesel(table_name = participants)]
pub struct ParticipantUpdate {
    pub name: Option<String>,
    pub pronouns: Option<String>,
}

// Response

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub room: Room,
    pub participants: Vec<Participant>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub participant: Participant,
    pub scores: Vec<Score>,
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
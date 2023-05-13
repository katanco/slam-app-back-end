// @generated automatically by Diesel CLI.

diesel::table! {
    participants (id) {
        id -> Text,
        name -> Text,
        pronouns -> Nullable<Text>,
        room_id -> Text,
    }
}

diesel::table! {
    participations (id) {
        id -> Text,
        performance_notes -> Nullable<Text>,
        performance_length_in_seconds -> Nullable<Int4>,
        deduction -> Nullable<Float4>,
        score -> Nullable<Float4>,
        performance_order -> Int4,
        round_id -> Text,
        participant_id -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        created -> Text,
        round_id_current -> Nullable<Text>,
        participation_id_current -> Nullable<Text>,
    }
}

diesel::table! {
    rounds (id) {
        id -> Text,
        round_number -> Int4,
        room_id -> Text,
    }
}

diesel::table! {
    scores (id) {
        id -> Text,
        value -> Float4,
        submitter_id -> Nullable<Text>,
        participation_id -> Text,
    }
}

diesel::joinable!(participants -> rooms (room_id));
diesel::joinable!(participations -> participants (participant_id));
diesel::joinable!(participations -> rounds (round_id));
diesel::joinable!(rooms -> participations (participation_id_current));
diesel::joinable!(scores -> participations (participation_id));

diesel::allow_tables_to_appear_in_same_query!(
    participants,
    participations,
    rooms,
    rounds,
    scores,
);

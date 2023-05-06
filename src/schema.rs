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
        performance_length_in_seconds -> Nullable<Integer>,
        deduction -> Nullable<Float>,
        score -> Nullable<Float>,
        performance_order -> Integer,
        round_id -> Text,
        participant_id -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        created -> Text,
    }
}

diesel::table! {
    rounds (id) {
        id -> Text,
        round_number -> Integer,
        room_id -> Text,
    }
}

diesel::table! {
    scores (id) {
        id -> Text,
        value -> Float,
        participation_id -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    participants,
    participations,
    rooms,
    rounds,
    scores,
);

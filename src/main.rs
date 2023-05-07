use std::env;

use axum::{
    routing::get,
    http::StatusCode,
    Json,
    Router,
    extract::{Query, Path},
    response::{ IntoResponse, Response },
};
use tower_http::{trace::TraceLayer, cors::CorsLayer, services::ServeDir, services::ServeFile};
use dotenv::dotenv;
use slam_app_rust_server::{db::*, models::*};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let conn_spec = "slam.db";
    // let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    // let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    run_migration();
    
    let serve_dir = ServeDir::new("./build").not_found_service(ServeFile::new("./build/index.html"));

    let app = Router::new()
        // .route("/", get(root))
        .route("/data/room", get(get_rooms).post(post_room))
        .route("/data/participant", get(get_participants).post(post_participant))
        .route("/data/score", get(get_scores).post(post_score))
        .route("/data/room/:id", get(get_room)).fallback_service(serve_dir)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

        let service_port = env::var("PORT").expect("PORT must be set");
        let service_endpoint = format!("{}:{}", "0.0.0.0", service_port);
    axum::Server
        ::bind(&service_endpoint.parse().unwrap())
        .serve(app.into_make_service()).await
        .unwrap();
}

async fn get_rooms() -> (StatusCode, Json<Vec<Room>>) {
    let rooms_result = retrieve_rooms(&mut establish_connection());
    return (StatusCode::OK, Json(rooms_result));
}
async fn get_room(Path(id): Path<String>) -> (StatusCode, Json<RoomResponse>) {
    let rooms_result = retrieve_room(&mut establish_connection(), id.as_str());
    return (StatusCode::OK, Json(rooms_result));
}
async fn post_room(Json(payload): Json<RoomRequest>) -> (StatusCode, Json<Room>) {
    let room_result = insert_room(&mut establish_connection(), &payload.name);
    return (StatusCode::CREATED, Json(room_result));
}

async fn post_participant(Json(payload): Json<ParticipantRequest>) -> Response {
    if let Some(participant_id) = payload.id {
        update_participant(
            &mut establish_connection(),
            participant_id,
            payload.name,
            payload.pronouns
        );
        return (StatusCode::OK, "Updated").into_response();
    } else {
        if let (Some(name), Some(room_id)) = (payload.name, payload.room_id) {
            let participant_result = insert_participant(
                &mut establish_connection(),
                &name,
                payload.pronouns,
                &room_id
            );
            return (StatusCode::CREATED, Json(participant_result)).into_response();
        } else {
            return (StatusCode::BAD_REQUEST, "send better params pls").into_response();
        }
    }
}
async fn post_score(Json(payload): Json<ScoreRequest>) -> (StatusCode, Json<Score>) {
    let score_result = insert_score(
        &mut establish_connection(),
        &payload.value,
        &payload.participant_id
    );

    return (StatusCode::CREATED, Json(score_result));
}
async fn get_participants(params: Query<ParticipantFilter>) -> Response {
    let result = retrieve_participants(&mut establish_connection(), &params.room_id);
    return (StatusCode::OK, Json(result)).into_response();
}

async fn get_scores(params: Query<ScoreFilter>) -> Response {
    let result = retrieve_scores(&mut establish_connection(), &params.participation_id);
    return (StatusCode::OK, Json(result)).into_response();
}
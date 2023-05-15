use std::{ env, sync::Arc };

use axum::{
    routing::{ get, patch, post },
    http::StatusCode,
    Json,
    Router,
    extract::{ Query, Path, ws::{ WebSocketUpgrade, WebSocket, Message }, State },
    response::{ IntoResponse, Response },
};
use tower_http::{ trace::TraceLayer, cors::CorsLayer, services::ServeDir, services::ServeFile };
use dotenv::dotenv;
use slam_app_rust_server::{ db::*, models::* };
use tokio::{sync::broadcast };
use futures::{sink::SinkExt, stream::StreamExt};

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_migration();

    let serve_dir = ServeDir::new("./build").not_found_service(
        ServeFile::new("./build/index.html")
    );

    let (tx, _rx) = broadcast::channel(100);
    let state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/data/room", get(get_rooms).post(post_room))
        .route("/data/room/:id", get(get_room).patch(patch_room).delete(delete_room))
        .route("/data/room/:id/advance", post(advance_room))
        .route("/data/room/:id/current", get(current_room))
        .route("/data/participant", get(get_participants).post(post_participant))
        .route("/data/participant/:id", patch(patch_participant).delete(delete_participant))
        .route("/data/round/:id", get(get_round))
        .route("/data/participation/:id", patch(patch_participation))
        .route("/data/score", get(get_scores).post(post_score))
        .route("/data/ws", get(websocket_handler))
        .with_state(state)
        .fallback_service(serve_dir)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let service_port = env::var("PORT").expect("PORT must be set");
    let service_endpoint = format!("{}:{}", "0.0.0.0", service_port);
    axum::Server
        ::bind(&service_endpoint.parse().unwrap())
        .serve(app.into_make_service()).await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn get_rooms() -> (StatusCode, Json<Vec<Room>>) {
    let rooms_result = retrieve_rooms(&mut establish_connection());
    return (StatusCode::OK, Json(rooms_result));
}
async fn get_room(Path(id): Path<String>) -> (StatusCode, Json<RoomResponse>) {
    let rooms_result = retrieve_room(&mut establish_connection(), id.as_str());
    return (StatusCode::OK, Json(rooms_result));
}
async fn post_room(Json(payload): Json<RoomRequest>) -> Response {
    if let Some(name) = payload.name {
        let room_result = insert_room(&mut establish_connection(), &name);
        return (StatusCode::CREATED, Json(room_result)).into_response();
    } else {
        return (StatusCode::BAD_REQUEST, "send better params pls").into_response();
    }
}

async fn patch_room(Path(id): Path<String>, Json(payload): Json<RoomRequest>) -> Response {
    update_room(&mut establish_connection(), id, payload.name);
    return (StatusCode::OK, "Updated").into_response();
}

async fn delete_room(Path(id): Path<String>) -> Response {
    remove_room(&mut establish_connection(), id);
    return (StatusCode::OK, "Deleted").into_response();
}

async fn patch_participant(
    Path(id): Path<String>,
    Json(payload): Json<ParticipantRequest>
) -> Response {
    update_participant(&mut establish_connection(), id, payload.name, payload.pronouns);
    return (StatusCode::OK, "Updated").into_response();
}

async fn delete_participant(Path(id): Path<String>) -> Response {
    remove_participant(&mut establish_connection(), id);
    return (StatusCode::OK, "Deleted").into_response();
}

async fn post_participant(Json(payload): Json<ParticipantRequest>) -> Response {
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
async fn post_score(Json(payload): Json<ScoreRequest>) -> (StatusCode, Json<Score>) {
    let score_result = insert_score(
        &mut establish_connection(),
        &payload.value,
        &payload.participation_id,
        &payload.submitter_id
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

async fn advance_room(Path(id): Path<String>, Json(payload): Json<Vec<Participant>>) -> Response {
    let result = create_next_round(&mut establish_connection(), &id, payload);

    return (StatusCode::CREATED, Json(result)).into_response();
}

async fn get_round(Path(id): Path<String>) -> Response {
    let result = retrieve_round(&mut establish_connection(), &id);
    return (StatusCode::OK, Json(result)).into_response();
}

async fn patch_participation(
    Path(id): Path<String>,
    Json(payload): Json<ParticipationRequest>
) -> Response {
    update_participation(&mut establish_connection(), id, payload.notes, payload.length);
    return (StatusCode::OK, "Updated").into_response();
}

async fn current_room(Path(id): Path<String>) -> Response {
    let room_result = retrieve_room(&mut establish_connection(), id.as_str());
    if let Some(round_id) = room_result.room.round_id_current {
        let result = retrieve_round(&mut establish_connection(), &round_id);
        return (StatusCode::OK, Json(result)).into_response();
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
}
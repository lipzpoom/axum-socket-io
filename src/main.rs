use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, warn};
use uuid::Uuid;

// Types for our application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    room: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    id: String,
    user_id: String,
    username: String,
    content: String,
    timestamp: u64,
    room: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JoinRoomData {
    room: String,
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    content: String,
    room: Option<String>,
}

// Application state
type AppState = Arc<RwLock<HashMap<String, User>>>;

async fn on_connect(socket: SocketRef, Data(data): Data<serde_json::Value>) {
    info!("Socket.IO connected: {} with data: {:?}", socket.id, data);

    // Send welcome message
    let _ = socket.emit(
        "welcome",
        serde_json::json!({
            "message": "Welcome to Axum Socket.IO server!",
            "socket_id": socket.id.to_string()
        }),
    );
}

async fn on_disconnect(socket: SocketRef) {
    info!("Socket.IO disconnected: {}", socket.id);
}

async fn on_join_room(socket: SocketRef, Data(data): Data<JoinRoomData>, state: State<AppState>) {
    info!("User {} joining room: {}", data.username, data.room);

    // Leave previous room if any
    let rooms = socket.rooms().unwrap_or_default();
    for room in rooms {
        if room != socket.id.to_string() {
            let _ = socket.leave(room.to_string());
        }
    }

    // Join new room
    let _ = socket.join(data.room.clone());

    // Update user in state
    let user = User {
        id: socket.id.to_string(),
        username: data.username.clone(),
        room: Some(data.room.clone()),
    };

    {
        let mut users = state.write().await;
        users.insert(socket.id.to_string(), user);
    }

    // Notify room about new user
    let _ = socket.to(data.room.clone()).emit(
        "user_joined",
        serde_json::json!({
            "username": data.username,
            "message": format!("{} joined the room", data.username)
        }),
    );

    // Send success response
    let _ = socket.emit(
        "room_joined",
        serde_json::json!({
            "room": data.room,
            "message": "Successfully joined room"
        }),
    );
}

async fn on_send_message(socket: SocketRef, Data(data): Data<ChatMessage>, state: State<AppState>) {
    let user = {
        let users = state.read().await;
        users.get(&socket.id.to_string()).cloned()
    };

    if let Some(user) = user {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            username: user.username.clone(),
            content: data.content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            room: data.room.or(user.room.clone()),
        };

        info!("Message from {}: {}", user.username, message.content);

        // Send message to room or broadcast
        if let Some(room) = &message.room {
            let _ = socket.to(room.clone()).emit("new_message", &message);
        } else {
            let _ = socket.broadcast().emit("new_message", &message);
        }
    } else {
        warn!("Message from unknown user: {}", socket.id);
    }
}

async fn on_leave_room(socket: SocketRef, state: State<AppState>) {
    let user = {
        let users = state.read().await;
        users.get(&socket.id.to_string()).cloned()
    };

    if let Some(user) = user {
        if let Some(room) = &user.room {
            let _ = socket.leave(room.clone());
            let _ = socket.to(room.clone()).emit(
                "user_left",
                serde_json::json!({
                    "username": user.username,
                    "message": format!("{} left the room", user.username)
                }),
            );
        }

        // Update user state
        let mut users = state.write().await;
        if let Some(user_state) = users.get_mut(&socket.id.to_string()) {
            user_state.room = None;
        }
    }
}

// HTTP handlers
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "axum-socketio",
        "version": "0.1.0"
    }))
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.read().await;
    Json(users.values().cloned().collect())
}

async fn get_user(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.read().await;
    match users.get(&user_id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting Axum Socket.IO server...");

    // Create application state
    let app_state: AppState = Arc::new(RwLock::new(HashMap::new()));

    // Create Socket.IO layer
    let (layer, io) = SocketIo::new_layer();

    // Setup Socket.IO event handlers
    let app_state_clone = app_state.clone();
    io.ns("/", move |socket: SocketRef| {
        info!("New socket connection: {}", socket.id);

        socket.on("connect", on_connect);
        socket.on("disconnect", on_disconnect);
        socket.on("join_room", {
            let state = app_state_clone.clone();
            move |socket, data| on_join_room(socket, data, State(state.clone()))
        });
        socket.on("send_message", {
            let state = app_state_clone.clone();
            move |socket, data| on_send_message(socket, data, State(state.clone()))
        });
        socket.on("leave_room", {
            let state = app_state_clone.clone();
            move |socket| on_leave_room(socket, State(state.clone()))
        });
    });

    // Build HTTP router
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users))
        .route("/api/users/:user_id", get(get_user))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    // Start server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server running on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

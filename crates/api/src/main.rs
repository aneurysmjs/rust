use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct QueryParams {
    limit: Option<u32>,
}

// Простое in-memory хранилище (в реальном приложении используйте базу данных)
static mut USERS: Vec<User> = Vec::new();
static mut NEXT_ID: u32 = 1;

#[tokio::main]
async fn main() {
    // Инициализируем тестовых пользователей
    unsafe {
        USERS.push(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });
        USERS.push(User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });
        NEXT_ID = 3;
    }

    let app = Router::new()
        .route("/", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📋 Available endpoints:");
    println!("  GET  / - Health check");
    println!("  GET  /users - Get all users");
    println!("  POST /users - Create new user");
    println!("  GET  /users/:id - Get user by ID");
    
    axum::serve(listener, app).await.unwrap();
}

// Health check endpoint
async fn health_check() -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("status".to_string(), "OK".to_string());
    response.insert("message".to_string(), "API is running!".to_string());
    Json(response)
}

// Получить всех пользователей
async fn get_users(Query(params): Query<QueryParams>) -> Json<Vec<User>> {
    unsafe {
        let users = if let Some(limit) = params.limit {
            USERS.iter().take(limit as usize).cloned().collect()
        } else {
            USERS.clone()
        };
        Json(users)
    }
}

// Получить пользователя по ID
async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
    unsafe {
        if let Some(user) = USERS.iter().find(|u| u.id == user_id) {
            Ok(Json(user.clone()))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}

// Создать нового пользователя
async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    unsafe {
        let new_user = User {
            id: NEXT_ID,
            name: payload.name,
            email: payload.email,
        };
        
        USERS.push(new_user.clone());
        NEXT_ID += 1;
        
        Ok(Json(new_user))
    }
}
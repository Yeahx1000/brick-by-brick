use actix_web::{get, post, web, HttpResponse, Responder};
// use serde_json::json;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::habit::Habit;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool, // Database connection pool
}

#[get("/habits")]
pub async fn get_habits(state: web::Data<AppState>) -> impl Responder {
    let habits = sqlx::query_as::<_, Habit>("SELECT * FROM habits")
        .fetch_all(&state.db_pool)
        .await;

    match habits {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/habits")]
async fn create_habit(new_habit: web::Json<Habit>, state: web::Data<AppState>) -> impl Responder {
    let mut habit = new_habit.into_inner();
    habit.id = Uuid::new_v4().to_string();
    habit.created_at = Utc::now();

    let result = sqlx::query(
        "INSERT INTO habits (id, name, description, completed, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(habit.id.clone())
    .bind(habit.name.clone())
    .bind(habit.description.clone())
    .bind(habit.completed)
    .bind(habit.created_at)
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(habit),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

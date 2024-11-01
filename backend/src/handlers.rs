use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::Habit;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool, // Database connection pool
}

#[get("/habits")]
pub async fn get_habits(db: &SqlitePool) -> Result<Vec<Habit>, sqlx::Error> {
    let habits = sqlx::query_as::<_, Habit>("SELECT * FROM habits")
        .fetch_all(db)
        .await?;
    Ok(habits)
}

#[post("/habits")]
async fn create_habit(new_habit: web::Json<Habit>, state: web::Data<AppState>) -> impl Responder {
    let mut habit = new_habit.into_inner();
    habit.id = Uuid::new_v4(); // Generate a new UUID for the habit
    habit.created_at = Utc::now(); // Set current time for created_at

    sqlx::query(
        "INSERT INTO habits (id, name, description, completed, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(habit.id)
    .bind(habit.name)
    .bind(habit.description)
    .bind(habit.completed)
    .bind(habit.created_at)
    .execute(&state.db_pool)
    .await
    .unwrap();

    HttpResponse::Created().json(habit)
}

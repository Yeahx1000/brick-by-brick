// not properly implemented yet, needs some work

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use sqlx::{Decode, SqlitePool};
use uuid::Uuid;
// use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, Sqlite};

#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Debug, Decode)]
pub struct Habit {
    pub id: String, // UUID as a String
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_habits).service(create_habit);
}

#[get("/habits")]
async fn get_habits(pool: web::Data<SqlitePool>) -> impl Responder {
    let habits = sqlx::query_as::<_, Habit>("SELECT * FROM habits")
        .fetch_all(pool.get_ref())
        .await;

    match habits {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(err) => {
            eprintln!("Error fetching habits: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch habits")
        }
    }
}

#[post("/habits")]
async fn create_habit(new_habit: web::Json<Habit>, pool: web::Data<SqlitePool>) -> impl Responder {
    let mut habit = new_habit.into_inner();
    habit.id = Uuid::new_v4().to_string();
    habit.created_at = Utc::now();

    sqlx::query(
        "INSERT INTO habits (id, name, description, completed, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&habit.id)
    .bind(&habit.name)
    .bind(&habit.description)
    .bind(habit.completed)
    .bind(habit.created_at)
    .execute(&**pool)
    .await
    .unwrap();

    HttpResponse::Created().json(habit)
}

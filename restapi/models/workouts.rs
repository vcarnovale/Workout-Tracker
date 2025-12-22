use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
struct workouts {
    pub userid: i32,
    pub lift_weight: i32,
    pub lift_reps: i32,
    pub workout_date: DateTime<Utc>,
    pub rep_range: String,
}
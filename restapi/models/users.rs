use chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(FromRow)]
struct user {
    pub user: i32,
    pub username: String,
}

#[derive(FromRow)]
struct userweight {
    pub userid: i32,
    pub uweight: i32,
    pub tstamp: DateTime<Utc>,
}

#[derive(FromRow)]
struct userheight {
    pub userid: i32,
    pub uheight: i32,
    pub tstamp: DateTime<Utc>,
}
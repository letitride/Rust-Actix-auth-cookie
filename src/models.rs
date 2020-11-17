use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable)]
pub struct Tree {
    pub id: i64,
    pub path: String,
}
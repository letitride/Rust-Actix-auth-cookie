use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

use diesel::deserialize::QueryableByName;
#[derive(Debug, Clone)]
pub struct Tree {
    pub id: i64,
    pub path: String,
    pub depth: i32,
}


impl QueryableByName<diesel::pg::Pg> for Tree {
    fn build<R: diesel::row::NamedRow<diesel::pg::Pg>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Tree {
            id: row.get("id")?,
            path: row.get("path")?,
            depth: row.get("depth")?,
        })
    }
}

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel::Queryable;
use diesel::Selectable;
use super::schema::tasks;

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(Sqlite))]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub description: &'a str,
    pub done: bool,
}


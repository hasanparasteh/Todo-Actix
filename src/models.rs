use diesel::{Queryable, Insertable, Identifiable};
use serde::{Serialize, Deserialize};
use crate::schema::todos;


#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub created: u64,
    pub ended: u64,
    pub schedule: u64,
    pub is_schedule: bool,
}

#[derive(Serialize, Deserialize)]
pub struct  TodoUpdate {
    pub title: Option<String>,
    pub status: Option<String>
}
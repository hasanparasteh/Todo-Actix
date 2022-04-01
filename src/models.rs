use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::todos;


#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub created: u64,
    pub ended: u64,
    pub schedule: u64,
    pub is_schedule: bool,
}

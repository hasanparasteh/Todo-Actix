
use diesel::{insert_into, OptionalExtension};
use diesel::{QueryDsl, RunQueryDsl, result::Error};
use crate::schema::todos::dsl::*;
use crate::{establish_connection, models::Todo};

pub fn get_todos() -> Result<Option<Vec<Todo>>, Error> {
    let connection = establish_connection();
    todos.get_results::<Todo>(&connection).optional()
}

pub fn get_todo(req_id: i32)->Result<Option<Todo>, Error> 
{
    let connection = establish_connection();
    
    todos
        .find(req_id)
        .get_result::<Todo>(&connection)
        .optional()
}

pub fn insert_todo(req_todo: &Todo) -> Result<usize, Error>
{
    let connection = establish_connection();
    
    insert_into(todos)
        .values(req_todo)
        .execute(&connection)
}

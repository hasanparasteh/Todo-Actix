use diesel::{insert_into, OptionalExtension, delete,ExpressionMethods, update};
use diesel::{QueryDsl, RunQueryDsl, result::Error};
use crate::models::TodoUpdate;
use crate::schema::todos::{dsl::*};
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

pub fn remove_todo(req_id: i32) -> Result<usize, Error>
{
    let connection = establish_connection();

    delete(todos)
        .filter(id.eq(req_id))
        .execute(&connection)
}

pub fn update_todo(req_id: i32, update_payload: TodoUpdate) -> Result<usize, Error> 
{
    let connection = establish_connection();

    let title_payload = match update_payload.title {
        Some(title_payload)=> title_payload,
        None => "".to_string()
    };

    let status_payload = match update_payload.status {
        Some(status_payload)=> status_payload,
        None => "".to_string()
    };

    let mut query = update(todos)
        .set(id.eq(req_id)).into_boxed();

    if title_payload.len() > 5 {
        query = query.filter(title.eq(title_payload));
    }

    if status_payload.len() > 3 {
        query = query.filter(status.eq(status_payload));
    }

    let result = query.execute(&connection);

    if let Ok(count) = result {
        if count == 0 {
            return Err(Error::NotFound);
        }
    }
    result
}

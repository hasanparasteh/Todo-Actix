use crate::{models::{Todo}, service::{get_todo, get_todos, insert_todo}};
use actix_web::{Responder, web, get, HttpResponse, post};
use serde_json::json;

#[get("/sample")]
pub async fn greet() -> impl Responder{
    web::Json(Todo {
            id: 1,
            status: "Done".to_string(),
            title: "Sample".to_string(),
            created: 1648759256,
            ended: 1648759256,
            is_schedule: true,
            schedule: 1648759256
        })
}

#[get("/")]
pub async fn root() -> impl Responder {
    web::Json(json!({
        "status": true,
        "message": "Go to /sample in order to see sample todo"
    }))
}


#[get("/todos")]
pub async fn todo_list() -> impl Responder {
   let fetched_todos = get_todos();

    if let Ok(fetched_todos) = fetched_todos {
        HttpResponse::Ok()
            .json(json!({
                "result": true,
                "todos": fetched_todos
            })
        )
    } else {
        HttpResponse::InternalServerError()
            .json(json!({
                "result":false
            }))
    }
}

#[get("/todos/{id}")]
pub async fn todo_item(path: web::Path<i32>) -> impl Responder {
    let req_id = path.into_inner();
    let fetched_todos = get_todo(req_id);

    match fetched_todos {
        Ok(fetched_todos) => {
            HttpResponse::Created().json(json!({
                "result": true,
                "todo": fetched_todos
            }))
        },
        Err(diesel::NotFound) => {
             HttpResponse::NotFound().json(json!({
                 "result": false
             }))
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({
                 "result": false
             }))
        }
    }
}

#[post("/todos")]
pub async fn todo_add(req_todo: web::Json<Todo>) -> impl Responder {
    let affected_rows = insert_todo(&req_todo);

    match affected_rows {
        Ok(_affected_rows) => {
            HttpResponse::Created().json(req_todo)
        },
        _ => {
            HttpResponse::InternalServerError().json(json!({
                "result": false,
                "message": "Database is down"
            }))
        }
    }
}
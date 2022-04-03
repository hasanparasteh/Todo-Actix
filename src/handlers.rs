use crate::{models::{Todo, TodoUpdate}, service::{get_todo, get_todos, insert_todo, remove_todo, update_todo}};
use actix_web::{Responder, web, get, HttpResponse, post, delete, put};
use diesel::result::Error::NotFound;
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
            if fetched_todos.is_none() {
                HttpResponse::NotFound().json(json!({
                    "result": false,
                }))
            }else{
                HttpResponse::Ok().json(json!({
                    "result": true,
                    "todo": fetched_todos
                }))
            } 
        },
        Err(NotFound) => {
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
    let req_todo: Todo = req_todo.into_inner();
    let affected_rows = insert_todo(&req_todo);

    match affected_rows {
        Ok(_affected_rows) => {
            HttpResponse::Created().json(req_todo)
        },
        Err(affected_rows) => {
            HttpResponse::InternalServerError().json(json!({
                "result": false,
                "message": affected_rows.to_string(),
            }))
        }
    }
}

#[delete("/todos/{id}")]
pub async fn todo_remove(path: web::Path<i32>) -> impl Responder {
    let req_id = path.into_inner();
    let affected_rows = remove_todo(req_id);

    match affected_rows {
        Ok(_affected_rows)=> {
            HttpResponse::Ok().json(json!({
                "result": true
            }))
        },
        Err(affected_rows) => {
            HttpResponse::InternalServerError().json(json!({
                "result": false,
                "message": affected_rows.to_string()
            }))
        }
    }
}

#[put("/todos/{id}")]
pub async fn todo_update_title(path: web::Path<i32>, body: web::Json<TodoUpdate>) -> impl Responder{
    let id = path.into_inner();
    let update_payload = body.into_inner();

    match update_payload.title {
        Some(ref title_req) => if title_req.len() < 3 {
            return HttpResponse::BadRequest().json(json!({
                "result": false,
                "message": "Title should be at least 3 letter"
            }));
        },
        None => (),
    };

    match update_payload.status {
        Some(ref status_req) => if status_req.len() < 3 {
            return HttpResponse::BadRequest().json(json!({
                "result": false,
                "message": "Status should be at least 3 letter"
            }));
        },
        None => (),
    };

    let affected_rows = update_todo(id, update_payload);

    match affected_rows {
        Ok(_affected_rows) => {
             HttpResponse::Ok().json(json!({
                "result": true
            }))
        },
        Err(NotFound) => {
            HttpResponse::NotFound().json(json!({
                "result": false,
                "message": "Id Not Found".to_string()
            }))
        },
        Err(_)=>{
            HttpResponse::InternalServerError().json(json!({
                "result": false,
                "message": "Something went wrong...".to_string()

            }))
        }
    }
}
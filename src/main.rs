
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

pub mod service;
pub mod handlers;

use actix_web::{App, HttpServer};
use crate::handlers::{root, greet, todo_list, todo_item, todo_add};

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server");

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(greet)
            .service(todo_list)
            .service(todo_item)
            .service(todo_add)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
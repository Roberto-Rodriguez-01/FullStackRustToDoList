#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};

mod models;
mod schema;

use models::{Task, NewTask};
use schema::tasks::dsl::*;

#[derive(Serialize)]
struct TasksResponse {
    tasks: Vec<Task>,
}

async fn get_tasks(connection: web::Data<Arc<Mutex<SqliteConnection>>>) -> impl Responder {
    let results = tasks
        .load::<Task>(&mut *connection.lock().unwrap())
        .expect("Error loading tasks");

    HttpResponse::Ok().json(TasksResponse { tasks: results })
}

#[derive(Deserialize)]
struct CreateTask {
    description: String,
}

async fn add_task(new_task: web::Json<CreateTask>, connection: web::Data<Arc<Mutex<SqliteConnection>>>) -> impl Responder {
    let new_task = NewTask {
        description: &new_task.description,
        done: false,
    };

    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(&mut *connection.lock().unwrap())
        .expect("Error adding task");

    HttpResponse::Ok().json("Task added")
}

async fn delete_task(req: HttpRequest, connection: web::Data<Arc<Mutex<SqliteConnection>>>) -> impl Responder {
    let task_id: i32 = req.match_info().query("id").parse().unwrap();

    diesel::delete(tasks.filter(id.eq(task_id)))
        .execute(&mut *connection.lock().unwrap())
        .expect("Error deleting task");

    HttpResponse::Ok().json("Task deleted")
}

#[derive(Deserialize)]
struct UpdateTask {
    id: i32,
    description: Option<String>,
    done: Option<bool>,
}

async fn update_task(task: web::Json<UpdateTask>, connection: web::Data<Arc<Mutex<SqliteConnection>>>) -> impl Responder {
    let target = tasks.filter(id.eq(task.id));

    if let Some(desc) = &task.description {
        diesel::update(target)
            .set(description.eq(desc))
            .execute(&mut *connection.lock().unwrap())
            .expect("Error updating task description");
    }

    if let Some(done_value) = task.done {
        diesel::update(target)
            .set(done.eq(done_value))
            .execute(&mut *connection.lock().unwrap())
            .expect("Error updating task status");
    }

    HttpResponse::Ok().json("Task updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = Arc::new(Mutex::new(establish_connection()));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(add_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            .route("/tasks", web::put().to(update_task))
            .service(Files::new("/", "./frontend/static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


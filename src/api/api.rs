use actix_web::{get, put, post, web, web::{Data, Json, ServiceConfig}, HttpResponse, delete};
use crate::repository::database::Database;
use crate::models::todo::{NewTodo, UpdateTodo};
use crate::not_found;


#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<NewTodo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let todo = db.get_todo_by_id(id.into_inner());

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => not_found().await.unwrap()
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let count = db.delete_todo_by_id(id.into_inner());

    match count {
        Some(count) => match count {
            1 => HttpResponse::Ok().json("Element was deleted".to_string()),
            _ => not_found().await.unwrap()
        },
        None => not_found().await.unwrap()
    }
}

#[put("/todos/{id}")]
pub async fn update_todo(db: Data<Database>, todo: Json<UpdateTodo>, id: web::Path<i32>) -> HttpResponse {
    let updated_todo = db.update_todo_by_id(id.into_inner(), todo.into_inner());
    match updated_todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => not_found().await.unwrap()
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_todo)
            .service(get_todo_by_id)
            .service(get_todos)
            .service(update_todo)
            .service(delete_todo_by_id)
    );
}
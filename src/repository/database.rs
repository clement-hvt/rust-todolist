use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::todo::{NewTodo, Todo, UpdateTodo};
use crate::models::schema::todos::dsl::*;
use std::fmt::Error;
use dotenv::dotenv;

pub type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_todo(&self, todo: NewTodo) -> Result<Todo, Error> {
        let new_todo = NewTodo {
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            ..todo
        };
        let todo = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    pub fn get_todo_by_id(&self, todo_id: i32) -> Option<Todo> {
        let todo = todos
            .find(todo_id)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading todo by id");
        todo
    }

    pub fn delete_todo_by_id(&self, todo_id: i32) -> Option<usize> {
        let count = diesel::delete(todos.find(todo_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo by id");
        println!("{count}");
        Some(count)
    }

    pub fn update_todo_by_id(&self, todo_id: i32, mut todo: UpdateTodo) -> Option<Todo> {
        todo.updated_at = Utc::now().naive_utc();
        let todo = diesel::update(todos.find(todo_id))
            .set(&todo)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error updating todo by id");
        todo
    }
}
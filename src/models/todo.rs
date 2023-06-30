use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = crate::models::schema::todos)]
pub struct Todo {
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(default)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::models::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(default)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug, Clone)]
#[diesel(table_name = crate::models::schema::todos)]
pub struct UpdateTodo {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub updated_at: chrono::NaiveDateTime,
}


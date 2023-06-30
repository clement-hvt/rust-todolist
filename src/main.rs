use actix_web::{App, get, HttpResponse, HttpServer, Responder, Result, web};
use serde::Serialize;

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub status: i32,
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        status: 200,
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        status: 404,
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = repository::database::Database::new();
    let app_data = web::Data::new(db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8888))?
        .run()
        .await
}

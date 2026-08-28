mod db;
mod handlers;
mod models;
mod sm2;

use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer};

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("not found")]
    NotFound,

    #[error("template error: {0}")]
    Template(#[from] askama::Error),

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound => HttpResponse::NotFound()
                .content_type("text/html")
                .body("<h1>404 — Not Found</h1><p>The page you are looking for does not exist.</p>"),
            AppError::BadRequest(msg) => HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("<h1>400 — Bad Request</h1><p>{msg}</p>")),
            _ => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<h1>500 — Server Error</h1><p>Something went wrong. Please try again.</p>"),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = rusqlite::Connection::open("ridge.db")
        .expect("failed to open database");

    db::init_db(&conn).expect("failed to initialize database schema");
    db::seed_cards(&conn).expect("failed to seed demo cards");

    let state = web::Data::new(AppState {
        db: Mutex::new(conn),
    });

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Ridge — listening on http://0.0.0.0:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(handlers::dashboard::dashboard))
            .route("/study", web::get().to(handlers::study::study))
            .route("/study/answer", web::post().to(handlers::study::answer))
            .route("/cards", web::get().to(handlers::cards::list))
            .route("/cards/new", web::get().to(handlers::cards::new_form))
            .route("/cards", web::post().to(handlers::cards::create))
            .route("/cards/{id}/edit", web::get().to(handlers::cards::edit_form))
            .route("/cards/{id}", web::post().to(handlers::cards::update))
            .route("/cards/{id}/delete", web::post().to(handlers::cards::delete))
            .route("/health", web::get().to(health))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

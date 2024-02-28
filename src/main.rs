mod db;
mod handlers;
mod middleware;
mod models;
mod pass;

use actix_web::{web, App, HttpServer};

use crate::handlers::{login_user, register_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

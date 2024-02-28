use crate::db::{create_user, init_db};
use crate::pass::hash_password;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

pub async fn register_user(data: web::Json<RegisterRequest>) -> impl Responder {
    let hashed_password = hash_password(&data.password);
    let conn = init_db().expect("Failed to initialize database");

    create_user(&conn, &data.username, &hashed_password, &data.email)
        .expect("Failed to create user");

    HttpResponse::Ok().body("User registered successfully")
}

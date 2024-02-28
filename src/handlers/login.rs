use crate::db::{get_user_by_username, init_db};
use crate::pass::verify_password;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_user(data: web::Json<LoginRequest>) -> impl Responder {
    let conn = init_db().expect("Failed to initialize database");

    if let Some(user) =
        get_user_by_username(&conn, &data.username).expect("Failed to retrieve user")
    {
        if verify_password(&data.password, &user.password_hash) {
            HttpResponse::Ok().body("Login successful")
        } else {
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

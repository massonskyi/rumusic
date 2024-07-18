use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::{manager::UserManager, signin, signup};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest{
    name: String,
    surname: String,
    age: i32,
    username: String,
    email: String,
    password: String,
    role: String,
    avatar: String,
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::resource("/signup")
        .route(web::post().to(signup::signup)),
    );
    cfg.service(
        web::resource("/signin")
        .route(web::post().to(signin::signin))
    );
    cfg.service(
        web::resource("users")
        .route(web::get().to(get_users))
        .route(web::post().to(create_user))
        // .route(web::get().to(get_user))
    );
}

async fn get_users(data: web::Data<UserManager>) -> impl Responder{
    let users = data.list_users().await;
    HttpResponse::Ok().json(users)
}
async fn create_user(data: web::Data<Arc<RwLock<UserManager>>>, req: web::Json<CreateUserRequest>) -> impl Responder {
    let mut manager = data.write().await;
    match manager.create_user(
        req.name.clone(),
        req.surname.clone(),
        req.age,
        req.username.clone(),
        req.email.clone(),
        req.password.clone(),
        req.role.clone(),
        req.avatar.clone()
    ).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}


async fn get_user(data: web::Data<Arc<RwLock<UserManager>>>, path: web::Path<(i32,)>) -> impl Responder {
    let manager = data.read().await;
    let user_id = path.0;

    match manager.get_user(user_id).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(format!("User with id {} not found", user_id)),
    }
}
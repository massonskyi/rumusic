use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::{manager::UserManager, sign_in, sign_up};

#[derive(Debug, Serialize, Deserialize,utoipa::ToSchema)]
pub struct CreateUserRequest{
    name: String,
    surname: String,
    age: i32,
    username: String,
    email: String,
    password: String,
    role: String,
    avatar: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Greeting {
    message: String,
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::resource("users")
        .route(web::get().to(get_users))
        .route(web::post().to(create_user))
        .route(web::get().to(get_user))
    )
    .route("/sign_up", 
        web::post().to(sign_up::sign_up)
    )
    .route("sign_in",
        web::post().to(sign_in::sign_in)
    );
}
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description= "successful response", body=UserManager)
    )
)]

// #[get("/users")]
async fn get_users(data: web::Data<UserManager>) -> impl Responder{
    let users = data.list_users().await;
    HttpResponse::Ok().json(users)
}
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "successful response", body = User)
    )
)]
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

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "successful response", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(data: web::Data<Arc<RwLock<UserManager>>>, path: web::Path<(i32,)>) -> impl Responder {
    let manager = data.read().await;
    let user_id = path.0;

    match manager.get_user(user_id).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(format!("User with id {} not found", user_id)),
    }
}
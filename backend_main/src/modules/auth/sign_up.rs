use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::{claims::Claims, manager::UserManager};


#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SignUpRequest{
    name: String, 
    surname: String, 
    age: i32, 
    username: String, 
    email: String, 
    password: String, 
    role: String, 
    avatar: String,
}


#[utoipa::path(
    post,
    path = "/sign_up",
    request_body = SignUpRequest,
    responses(
        (status = 200, description = "Successful response", body = String), // JWT token as a string
        (status = 401, description = "Unauthorized response", body = String),
        (status = 403, description = "Forbidden response", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn sign_up(
    // Login Request: web::Json<LoginRequest> -> impl Responder
    user_manager: web::Data<Arc<RwLock<UserManager>>>, 
    req: web::Json<SignUpRequest>
) -> impl Responder{

    let hashed_password = match hash(&req.password, 10) {
        Ok(hashed) => {
            println!("{}", hashed);
            hashed
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Clone necessary fields from SignUpRequest
    let username = req.username.clone();
    let email = req.email.clone();
    let role = req.role.clone();
    let avatar = req.avatar.clone();
    println!("{username} {email} {role}, {avatar} {0} {1} {2}", req.name.clone(),req.surname.clone(), req.age);
    // Obtain mutable access to UserManager
    let mut manager = user_manager.write().await;

    // Create the user in the database
    let user = match manager.create_user(
        req.name.clone(),     // Clone other fields as needed
        req.surname.clone(),
        req.age,
        username,
        email,
        hashed_password,
        role,
        avatar,
    ).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Generate JWT token for the newly created user
    let claims = Claims {
        sub: user.id,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // Token expires in 24 hours
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"test"), // Replace with your actual secret key
    ) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    println!("{token}");
    // Return token as JSON response
    HttpResponse::Ok().json(token)
}
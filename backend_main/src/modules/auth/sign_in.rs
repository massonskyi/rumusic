use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{claims::Claims, manager::UserManager};  // Assuming UserManager is implemented elsewhere


#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    username: String,
    password: String,
} // Note: username and password are not required fields

#[utoipa::path(
    get,
    request_body = LoginRequest,
    path = "/sign_in",
    responses(
        (status = 200, description= "successful response", body=LoginRequest),
        (status = 401, description= "unauthorized response", body=String),
        (status = 403, description= "forbidden response", body=String),
        (status = 500, description= "internal server error", body=String),
    )
)]
pub async fn sign_in(
    user_manager: web::Data<Arc<RwLock<UserManager>>>, 
    req: web::Json<LoginRequest>
) -> impl Responder {
    // Obtain mutable access to UserManager
    let manager = user_manager.read().await;

    // Retrieve user by username (assuming UserManager has a method to find user by username)
    let user = match manager.get_user_by_username(&req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().finish(), // User not found
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Verify password using bcrypt
    let password_match = match verify(&req.password, &user.hash_password) {
        Ok(matched) => matched,
        Err(_) => return HttpResponse::Unauthorized().finish(), // Error verifying password
    };

    if !password_match {
        return HttpResponse::Unauthorized().finish(); // Incorrect password
    }

    // Generate JWT token for the authenticated user
    let claims = Claims {
        sub: user.id,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // Token expires in 24 hours
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"secret"), // Replace with your actual secret key
    ) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Return token as JSON response
    HttpResponse::Ok().json(token)
}
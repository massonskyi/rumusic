use actix_web::{HttpMessage, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use hmac::Hmac;
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use actix_web::dev::ServiceRequest;
use actix_web::error::Error;

use actix_web_httpauth::extractors::bearer::{BearerAuth};
use actix_web::error::ErrorUnauthorized;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub async fn validate(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let token_string = credentials.token();

    let validation = Validation::new(Algorithm::HS256);
    
    let claims_result = decode::<Claims>(
        &token_string,
        &key,
        &validation
    );

    match claims_result {
        Ok(token_data) => {
            // Insert claims into the request extensions
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => {
            // Handle invalid token scenario
            Err((ErrorUnauthorized("Invalid token").into(), req))
        }
    }
}
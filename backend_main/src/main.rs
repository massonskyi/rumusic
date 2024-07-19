use std::sync::Arc;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use utoipa::OpenApi;

mod routes;
mod modules;
mod db;
use db::establish_connection;

use modules::auth::{
    self,
    claims::validate,
    manager::UserManager};

use crate::auth::sign_in::sign_in;
use crate::auth::sign_up::sign_up;
use routes::{track_routes, playlist_routes};
async fn render_start_page() -> &'static str {
    "Hello on start page!\ngo to \\tracks or \\users or \\playlists"
}

async fn index() -> impl Responder {
    
    HttpResponse::Ok().body(render_start_page().await)
}


#[derive(OpenApi)]
#[openapi(
    paths(
        modules::auth::sign_in::sign_in,
        modules::auth::sign_up::sign_up
    ),
    components(
        schemas(
            modules::auth::sign_up::SignUpRequest,
            modules::auth::sign_in::LoginRequest
        )
    )
)]
struct ApiDoc;

async fn get_openapi() -> HttpResponse {
    HttpResponse::Ok().json(ApiDoc::openapi())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = 8080;
    let db_url = establish_connection();
    let bearer_middleware = HttpAuthentication::bearer(validate);
    let user_manager = Arc::new(tokio::sync::RwLock::new(
        UserManager::new(&db_url).await.expect("Failed to create UserManager")
    ));
    println!("Server started at http://{host}:{port}/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_manager.clone()))
            .wrap(
                Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
            )
            .configure(modules::auth::handler::init)
            .route("/", web::get().to(index))
            .configure(track_routes::init)
            .configure(playlist_routes::init)
            .route("/api-doc/openapi.json", web::get().to(get_openapi))

    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
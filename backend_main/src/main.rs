use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod routes;
mod modules;
mod db;
use db::establish_connection;
use modules::auth::{self, manager::UserManager};
use routes::{user_routes, track_routes, playlist_routes};


async fn render_start_page() -> &'static str {
    "Hello on start page!\ngo to \\tracks or \\users or \\playlists"
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(render_start_page().await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = 8080;
    let db_url = establish_connection();

    // Создание экземпляра UserManager
    let user_manager = UserManager::new(&db_url).await.expect("Failed to create UserManager");
    
    println!("Server started at http://{host}:{port}/");

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(user_manager.clone()))
            .route("/", web::get().to(index))
            .configure(auth::handler::init)
            .configure(track_routes::init)
            .configure(playlist_routes::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
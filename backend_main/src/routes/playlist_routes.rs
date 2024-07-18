use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/playlists")
            .route(web::get().to(get_playlists))
            .route(web::post().to(create_playlist)),
    );
}

async fn get_playlists() -> impl Responder {
    HttpResponse::Ok().body("Get all playlists")
}

async fn create_playlist() -> impl Responder {
    HttpResponse::Ok().body("Create a playlist")
}
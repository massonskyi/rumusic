use actix_web::{web, HttpResponse, Responder};


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tracks")
        .route(web::get().to(get_tracks))
        .route(web::post().to(create_track)),
    );
}

async fn get_tracks() -> impl Responder {
    HttpResponse::Ok().json(vec!["track1", "track2", "track3"])
}

async fn create_track() -> impl Responder {
    HttpResponse::Ok().json("track created!")
}
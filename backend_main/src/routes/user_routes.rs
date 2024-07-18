use actix_web::{web, HttpResponse, Responder};


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
        .route(web::get().to(get_users))
        .route(web::post().to(create_user)),
    );   
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec!["user1", "user2"])
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().json("user created")
}

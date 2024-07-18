use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

use std::env;
use dotenvy::dotenv;


// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


// pub fn establish_connection() -> DbPool {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     println!("Started connected to database on url{database_url}");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     r2d2::Pool::builder()
//             .build(manager)
//             .expect("Failed to create pool.")
// }
pub fn establish_connection() -> String {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("DB_HOST").expect("Host not found");
    let port = env::var("DB_PORT").expect("Port not found");
    println!("Started connected to database on url{database_url} {host} {port}");
    database_url
}
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(let_chains)]
mod controller;
mod model;
mod schema;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use rocket::fs::FileServer;

use crate::model::session::SessionManager;

#[macro_use]
extern crate rocket;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let pool = establish_pool();
    let session_manager = SessionManager::new();

    let server = rocket::build()
        .manage(pool)
        .manage(session_manager)
        // .mount("/", FileServer::from("./static/"));
        .mount("/", FileServer::from("./web-client/dist/"));
    controller::init_pages(server).launch().await?;
    Ok(())
}

fn establish_pool() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Could not build connection pool")
}

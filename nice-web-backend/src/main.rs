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
        .mount("/", FileServer::from("./static/"));
    controller::init_pages(server).launch().await?;
    Ok(())
}

fn establish_pool() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    println!("no problem by now");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        // .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

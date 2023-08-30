#![feature(proc_macro_hygiene, decl_macro)]

mod controller;
mod model;
mod schema;
mod view;

use diesel::{
    PgConnection,
    r2d2::{
        Pool,
        ConnectionManager,
    }
};
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let pool = establish_pool();

    let server = rocket::build()
        .manage(pool);
    controller::init_pages(server).launch().await?;
    Ok(())
}

fn establish_pool() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
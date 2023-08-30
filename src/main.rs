#![feature(proc_macro_hygiene, decl_macro)]

mod controller;
mod model;
mod schema;
mod view;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let server = rocket::build();
    controller::init_pages(server).launch().await?;
    Ok(())
}

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = "";
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
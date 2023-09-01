mod routes;

use rocket::{Build, Rocket};

use crate::controller::routes::signin::signin;
use crate::controller::routes::signup::signup;
use crate::controller::routes::signout::signout;
use crate::controller::routes::posts::showusers::showusers;
use crate::controller::routes::posts::createpost::createpost;
use crate::controller::routes::posts::removepost::removepost;
use crate::controller::routes::posts::bookmarkpost::bookmarkpost;

pub fn init_pages(server: Rocket<Build>) -> Rocket<Build> {
    let server = server.mount("/", routes![signin, signout, signup, showusers, createpost, removepost, bookmarkpost]);
    server
}

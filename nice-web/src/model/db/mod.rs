use std::sync::Arc;
use crate::schema::{users, post};
use diesel::associations::HasTable;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

pub use super::super::schema::*;

extern crate bcrypt;

use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};
use rocket::http::Status;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use crate::model::session::SessionManager;



pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = ""; // todo where
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[derive(Debug)]
pub enum TempEnumForUser {
    SessionExpired
}

pub enum CheckUserResult {
    Ok,
    NoSuchUser,
    NoSuchPassword
}

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TempEnumForUser;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // it's better to put a session manager in Arc (?) todo
        let sessions = request.guard::<&State<Arc<SessionManager>>>().await.expect("unexpected behavior");

        if let Some(session) = request.headers().get_one("session-token") {
            // there's a one problem: every route will get cached user info, posiibly its not so bad
            if let Some(user) = sessions.get_session(session) {
                return Outcome::Success(user)
            }
        }
        Outcome::Failure((Status::Unauthorized, TempEnumForUser::SessionExpired))
    }
}

pub fn is_user_password_good_and_user_exists(
    mut connection: PgConnection,
    user_name: &str,
    user_password: &str,
) -> CheckUserResult {
    let users: Vec<User> = users::table
        .filter(users::username.eq(user_name))
        .limit(1)
        .select(User::as_select())
        .load(&mut connection)
        .expect("problems with getting users");
    let user = users.first();
    match user {
        None => {
            return CheckUserResult::NoSuchUser;
        }
        Some(user) => {
            let verify_result = verify(user_password, user.password_hash.as_str());
            match verify_result {
                Ok(false) | Err(_) => {
                    return CheckUserResult::NoSuchPassword;
                }
                Ok(true) => {
                    return CheckUserResult::Ok;
                }
            }
        }
    }
}

pub enum CreateUserResult {
    Ok,
    SuchUserExists,
    PasswordNotValidForHash,
}

pub fn create_user_if_unique(
    mut connection: PgConnection,
    user_name: &str,
    user_password: &str,
) -> CreateUserResult {
    let users: Vec<User> = users::table
        .filter(users::username.eq(user_name))
        .limit(1)
        .select(User::as_select())
        .load(&mut connection)
        .expect("problems with getting users");
    let user = users.first();
    match user {
        None => {
            let pass_hash = hash(user_password, DEFAULT_COST);
            match pass_hash {
                Ok(pass) => {
                    diesel::insert_into(users::table)
                        .values((
                            users::username.eq(user_name),
                            users::password_hash.eq(pass.as_str()),
                        ))
                        .execute(&mut connection)
                        .expect("problems with inserting new user");
                }
                Err(_) => {
                    return CreateUserResult::PasswordNotValidForHash;
                }
            }
        }
        Some(_) => {
            return CreateUserResult::SuchUserExists;
        }
    }
    return CreateUserResult::Ok;
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::post)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub is_bookmarked: bool
}
pub fn show_user_posts(mut connection: PgConnection, user_id: i32) -> Vec<Post> {
    let user_posts = post::table.filter(post::user_id.eq(user_id)).order(post::is_bookmarked.desc()).load(&mut connection)
        .expect("problems with getting user posts"); // todo possibly its bad idea and Result is more preferable
    user_posts
}

pub fn create_user_post(mut connection: PgConnection, user_id: i32, title: String, description: String) {
    diesel::insert_into(post::table)
        .values((
            post::user_id.eq(user_id),
            post::title.eq(title),
            post::description.eq(description)
        ))
        .execute(&mut connection)
        .expect("problems with inserting new post");
}

pub fn delete_post(mut connection: PgConnection, post_id: i32) {
    diesel::delete(post::table.filter(post::post_id.eq(post_id))).execute(&mut connection)
        .expect("problems with deleting post");
}

pub fn set_bookmark_post(mut connection: PgConnection, post_id: i32, boool: bool) {
    diesel::update(post::table.filter(post::post_id.eq(post_id)))
        .set(post::is_bookmarked.eq(boool))
        .execute(&mut connection)
        .expect("problems with deleting post");
}
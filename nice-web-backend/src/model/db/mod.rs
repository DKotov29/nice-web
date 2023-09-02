use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::pg::Pg;
use diesel::{
    Connection, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::Serialize;

pub use super::super::schema::*;
use crate::schema::{post, users};

extern crate bcrypt;

pub enum CheckUserResult {
    Ok(User),
    NoSuchUser,
    NoSuchPassword,
}

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
}

pub fn is_user_password_good_and_user_exists<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
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
                    return CheckUserResult::Ok(user.clone());
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

pub fn create_user_if_unique<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
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
                        .values((users::username.eq(user_name), users::password_hash.eq(pass.as_str())))
                        .execute(&mut connection)
                        .expect("problems with inserting new user");
                    CreateUserResult::Ok
                }
                Err(_) => CreateUserResult::PasswordNotValidForHash,
            }
        }
        Some(_) => CreateUserResult::SuchUserExists,
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::post)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub bookmarked: bool,
}

pub fn show_user_posts<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
    user_id: i32,
) -> Vec<Post> {
    post::table
        .filter(post::user_id.eq(user_id))
        .order(post::bookmarked.desc())
        .load(&mut connection)
        .unwrap_or(vec![])
}

pub fn create_user_post<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
    user_id: i32,
    title: String,
    description: String,
) {
    diesel::insert_into(post::table)
        .values((
            post::user_id.eq(user_id),
            post::title.eq(title),
            post::description.eq(description),
        ))
        .execute(&mut connection)
        .expect("problems with inserting new post");
}

pub fn delete_post<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
    post_id: i32,
    user_id: i32,
) {
    diesel::delete(
        post::table
            .filter(post::post_id.eq(post_id))
            .filter(post::user_id.eq(user_id)),
    )
    .execute(&mut connection)
    .expect("problems with deleting post");
}

pub fn set_bookmark_post<T: Connection<Backend = Pg> + diesel::connection::LoadConnection>(
    mut connection: T,
    post_id: i32,
    user_id: i32,
    boool: bool,
) {
    diesel::update(
        post::table
            .filter(post::post_id.eq(post_id))
            .filter(post::user_id.eq(user_id)),
    )
    .set(post::bookmarked.eq(boool))
    .execute(&mut connection)
    .expect("problems with deleting post");
}

use crate::schema::users;
use diesel::associations::HasTable;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

pub use super::super::schema::*;

extern crate bcrypt;

use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = ""; // todo where
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub enum CheckUserResult {
    Ok,
    NoSuchUser,
    NoSuchPassword,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
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

// todo show user posts, create user post, delete user post

use std::fmt::{Display, Formatter};

use gloo::net::http::{Request, Response};

use crate::api::types::{CreatePost, Credentials, ErrorResponse, Post, PostsResponse, Session};

pub mod types;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Api {
    pub session: Option<Session>,
}

#[derive(Debug)]
pub enum ApiError {
    RequestError(gloo::net::Error),
    ResponseError(gloo::net::Error),
    ApiError(u16, String),
    EmptySession,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::RequestError(err) => {
                writeln!(f, "{}", err)
            }
            ApiError::ResponseError(err) => {
                writeln!(f, "{}", err)
            }
            ApiError::ApiError(status, err) => {
                writeln!(f, "{}: {}", status, err)
            }
            ApiError::EmptySession => {
                writeln!(f, "You aren't signed in")
            }
        }
    }
}

impl Api {
    // probably it can be implemented by passing the base host value by the env! macros.
    // and the value will be passed by build.rs script
    // rust compiler will set base host
    pub fn new(session: Option<Session>) -> Self {
        Self { session }
    }

    pub async fn sign_up(&self, credentials: Credentials) -> Result<(), ApiError> {
        // Ok(())
        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1:8000/signup")
            .json(&credentials)
            .map_err(|err| ApiError::RequestError(err))?
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        if !status.eq(&200) {
            return Self::read_error(response).await;
        }
        Ok(())
    }

    pub async fn sign_in(&self, credentials: Credentials) -> Result<Session, ApiError> {
        // Ok(Session("hello".to_string()))

        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1:8000/signin")
            .json(&credentials)
            .map_err(|err| ApiError::RequestError(err))?
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        if !status.eq(&200) {
            return Self::read_error(response).await;
        }

        let session = response
            .json::<Session>()
            .await
            .map_err(|err| ApiError::ApiError(status, err.to_string()))?;

        Ok(session)
    }

    pub async fn sign_out(&self) -> Result<(), ApiError> {
        let session = if let Some(session) = &self.session {
            session.token.clone()
        } else {
            return Err(ApiError::EmptySession);
        };

        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1:8000/signout")
            .header("session-token", &session)
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        if !status.eq(&200) {
            return Self::read_error(response).await;
        }
        Ok(())
    }

    // TODO: read posts
    pub async fn get_posts(&self) -> Result<Vec<Post>, ApiError> {
        let session = if let Some(session) = &self.session {
            session.token.clone()
        } else {
            return Err(ApiError::EmptySession);
        };

        let response = Request::get("http://127.0.0.1:8000/showusers")
            .header("session-token", &session)
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        if !response.status().eq(&200) {
            return Self::read_error(response).await;
        }

        let posts = response
            .json::<PostsResponse>()
            .await
            .map_err(|err| ApiError::ApiError(response.status(), err.to_string()))?;
        Ok(posts.posts)
    }

    pub async fn create_post(&self, post: CreatePost) -> Result<(), ApiError> {
        let session = if let Some(session) = &self.session {
            session.token.clone()
        } else {
            return Err(ApiError::EmptySession);
        };

        let response = Request::post("http://127.0.0.1:8000/createpost")
            .header("session-token", &session)
            .json(&post)
            .unwrap()
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        if !response.status().eq(&200) {
            return Self::read_error(response).await;
        }

        Ok(())
    }

    pub async fn delete_post(&self, post_id: i32) -> Result<(), ApiError> {
        let session = if let Some(session) = &self.session {
            session.token.clone()
        } else {
            return Err(ApiError::EmptySession);
        };

        let response = Request::delete(&format!("http://127.0.0.1:8000/removepost/{}", post_id))
            .header("session-token", &session)
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        if !response.status().eq(&200) {
            return Self::read_error(response).await;
        }

        Ok(())
    }

    pub async fn bookmark_post(&self, post_id: i32, bookmark: bool) -> Result<(), ApiError> {
        let session = if let Some(session) = &self.session {
            session.token.clone()
        } else {
            return Err(ApiError::EmptySession);
        };

        let response = Request::post(&format!("http://127.0.0.1:8000/bookmarkpost/{}/{}", post_id, bookmark))
            .header("session-token", &session)
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        if !response.status().eq(&200) {
            return Self::read_error(response).await;
        }

        Ok(())
    }

    async fn read_error<T>(response: Response) -> Result<T, ApiError> {
        let status = response.status();
        let err = response
            .json::<ErrorResponse>()
            .await
            .map_err(|err| ApiError::ApiError(status, err.to_string()))?;
        return Err(ApiError::ApiError(status, err.error));
    }
}

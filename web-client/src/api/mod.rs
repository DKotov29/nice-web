use gloo::net::http::{Request, Response};

use crate::api::types::{Credentials, ErrorResponse, Post, PostsResponse, Session};

mod types;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Api {
    pub session: Option<Session>,
}

#[derive(Debug)]
pub enum ApiError {
    RequestError(gloo::net::Error),
    ResponseError(gloo::net::Error),
    ApiError(u16, String),
}

impl Api {
    pub fn new(session: Option<Session>) -> Self {
        Self { session }
    }

    pub async fn sign_up(&self, credentials: Credentials) -> Result<(), ApiError> {
        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1/signup")
            .json(&credentials)
            .map_err(|err| ApiError::RequestError(err))?
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        if !status.eq(&200) {
            return Self::read_error(response).await
        }
        Ok(())
    }

    pub async fn sign_in(&mut self, credentials: Credentials) -> Result<Session, ApiError> {
        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1/signin")
            .json(&credentials)
            .map_err(|err| ApiError::RequestError(err))?
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        let session = response
            .json::<Session>()
            .await
            .map_err(|err| ApiError::ApiError(status, err.to_string()))?;

        self.session = Some(session.clone());
        Ok(session)
    }

    pub async fn sign_out(&self) -> Result<(), ApiError> {
        // TODO: find out how to replace domain from .env file
        let response = Request::post("http://127.0.0.1/signout")
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let status = response.status();

        if !status.eq(&200) {
            return Self::read_error(response).await
        }
        Ok(())
    }

    // TODO: read posts

    async fn read_error(response: Response) -> Result<(), ApiError> {
        let status = response.status();
        let err = response
            .json::<ErrorResponse>()
            .await
            .map_err(|err| ApiError::ApiError(status, err.to_string()))?;
        return Err(ApiError::ApiError(status, err.error));
    }

    async fn get_posts() -> Result<Vec<Post>, ApiError> {
        let response = Request::post("http://127.0.0.1/getusers")
            .send()
            .await
            .map_err(|err| ApiError::ResponseError(err))?;

        let posts = response.json::<PostsResponse>().await.map_err(|err| ApiError::ApiError(response.status(), err.to_string()))?;
        Ok(posts.posts)
    }
}

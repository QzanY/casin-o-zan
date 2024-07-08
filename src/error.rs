use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};

#[derive(Clone, Debug)]
pub enum ServerError
{
    InvalidToken,
    UserNotFound,
    DatabaseError,
    UserExists,
    NoCookie,
    ExpiredToken,
    NotLoggedIn,
    NotEnoughFunds,
    OtherError
}

impl std::fmt::Display for ServerError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{:?}",self)
    }
}

impl IntoResponse for ServerError
{

    fn into_response(self) -> Response
    {
        let body = Html(self.to_string());
        let mut response = (StatusCode::INTERNAL_SERVER_ERROR, body).into_response();
        response.extensions_mut().insert(self);
        response
    }
}

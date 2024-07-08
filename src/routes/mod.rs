use std::sync::Arc;

use axum::routing::post;
use axum::Router;

use crate::AppState;
use crate::routes::register_route::{create_user, delete_user};

pub mod register_route;
pub mod login_route;
pub mod auth;

pub const AUTH_TOKEN: &str = "auth_token";
pub const COOKIE_SECRET: &str = "28+3";

pub fn user_routes(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/register", post(create_user).delete(delete_user))
        .route("/login", post(login_route::login_user))
        .with_state(state)
}

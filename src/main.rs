#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(unused_variables)]

use std::sync::Arc;
use axum::{middleware, response::Html, routing::get, Router};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

mod store;
mod routes;
mod error;
mod models;
mod context;
mod consts;
use error::ServerError;
use tower_cookies::CookieManagerLayer;

pub struct AppState
{
    db: Pool<Postgres>,
}
#[tokio::main]
async fn main() -> Result<(), ServerError>
{
    dotenv().ok();
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&consts::get_database_url())
        .await
    {
        Ok(pool) => pool,
        Err(_) => return Err(ServerError::DatabaseError),
    };
    
    let auth_needed_routes =  Router::new()
                                .route("/games",get(|| async {Html("<h1>Roll the dice!</h1>")}))
                                .route_layer(middleware::from_fn(routes::auth::check_auth));

    let main_routes = Router::new()
        .route("/", get(|| async {Html("<h1>Hello, World!</h1>")}))
        .merge(store::store_routes(Arc::new(AppState { db: pool.clone() })))
        .merge(auth_needed_routes)
        .layer(middleware::from_fn_with_state(Arc::new(AppState { db: pool.clone() }), routes::auth::context_resolver))
        .merge(routes::user_routes(Arc::new(AppState { db: pool.clone() })))
        .layer(CookieManagerLayer::new());
    

    let addr = consts::SOCKET_ADDR;
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener,main_routes).await.unwrap();

    Ok(())


    
}


use std::sync::Arc;

use axum::{routing::get, Router};

use crate::AppState;

mod store;

pub fn store_routes(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/store/user", get(store::list_items))
        .route("/store", get(store::list_store).post(store::buy_item).delete(store::sell_item) )
        .with_state(state)
}
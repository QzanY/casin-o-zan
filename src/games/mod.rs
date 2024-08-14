use std::sync::Arc;

use axum::{routing::get, Router};

use crate::AppState;

mod dice;

pub fn game_routes(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/games/dice", get(dice::dice_game))
        .with_state(state)
}
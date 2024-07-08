use std::{str::FromStr, sync::Arc};

use axum::{async_trait, body::Body, extract::State, http::Request, middleware::Next, response::Response, RequestPartsExt};
use tower_cookies::Cookies;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::{context::Context, error::ServerError, models::{Item, Token}, AppState};
use axum::{extract::FromRequestParts, http::request::Parts};
use sqlx::types::chrono;

pub async fn check_auth(context: Result<Context,ServerError>, req: Request<Body>, next: Next) -> Result<Response<Body>,ServerError>
{
    context?;
    Ok(next.run(req).await)
}

pub async fn context_resolver(
    state: State<Arc<AppState>>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response<Body>,ServerError>
{
    println!(">>> RESOLVING CONTEXT\n");
        let token = cookies.get(super::AUTH_TOKEN).ok_or(ServerError::NoCookie).unwrap()
            .value().to_string();

        let mut valid = Validation::default();
        valid.set_required_spec_claims(&[""]);
        let credidentials = decode::<Token>(&token, &DecodingKey::from_secret(super::COOKIE_SECRET.as_ref()), &valid)
            .map_err(|_| ServerError::InvalidToken).unwrap();

        let credidentials = credidentials.claims;

        if chrono::Utc::now().timestamp() - credidentials.time > 3600
        {
            return Err(ServerError::ExpiredToken);
        }
        
        let result = sqlx::query!(
            "
            SELECT balance,items
            FROM public.users
            WHERE name = $1
            ",
            credidentials.name
        ).fetch_one(&state.db).await;

        let val = match result
        {
            Ok(user) => {
                let balance = user.balance.unwrap();
                let items = serde_json::from_value::<Vec<(Item,i32)>>(user.items.clone().unwrap()).unwrap();
                Ok(Context::new(credidentials.name, balance, items))
                
            },
            Err(_) => Err(ServerError::UserNotFound)
        };
        req.extensions_mut().insert(val);
        Ok(next.run(req).await)


}

// TODO: Add cookie deletion and proper validation

#[async_trait]
impl<S: Send+Sync> FromRequestParts<S> for Context
{
    type Rejection = ServerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, ServerError>
    {   
        println!(">>> IN CONTEXT\n");
        parts
            .extensions
            .get::<Result<Context,ServerError>>()
            .ok_or(ServerError::NotLoggedIn)?
            .clone()
    }
}
use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::from_value;
use sqlx::types::chrono;
use tower_cookies::{Cookie, Cookies};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{error::ServerError, models::{Token, User, UserRequest}, AppState};

use super::AUTH_TOKEN;

pub async fn login_user(cookies:Cookies, state: State<Arc<AppState>>, Json(userr): Json<UserRequest>) -> Result<Json<User>,ServerError>
{
    let hashed_password = md5::compute(&userr.password);
    println!(">>> LOGIN ATTEMPT");
    let result = sqlx::query!(
        "
        SELECT name, password, balance, items
        FROM users
        WHERE name = $1 AND password = $2
        ",
        userr.name,
        format!("{:x}", hashed_password)
    ).fetch_one(&state.db).await;
    
    match result
    {
        Ok(user) => 
        {
            println!(">>> LOGIN SUCCESS\n");
            let name = user.name;
            let balance = user.balance;
            let items = user.items;
            let current_time = chrono::Utc::now().timestamp();
            let pre_token = Token
            {
                name: name.clone().unwrap(),
                time: current_time,
            };
            let token = encode(&Header::default(),&pre_token,&EncodingKey::from_secret(super::COOKIE_SECRET.as_ref())).unwrap();
            cookies.add(Cookie::new(AUTH_TOKEN,token));
            let response = User
            {
                name: name.unwrap(),
                password: "REDACTED".to_string(),
                balance: balance.unwrap(),
                items: from_value(items.unwrap()).unwrap_or(vec![]),
            };
            Ok(Json(response))
            
        },
        Err(_) => {
            println!(">>> LOGIN FAILURE\n");
            Err(ServerError::UserNotFound)
        },
    }
}

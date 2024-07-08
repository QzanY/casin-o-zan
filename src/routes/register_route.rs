use std::sync::Arc;
use md5;

use axum::{extract::State, Json};
use crate::{error::ServerError, models::UserRequest, AppState};
use crate::models::User;
use serde_json::json;


pub async fn create_user(state: State<Arc<AppState>>, Json(user): Json<UserRequest>) -> Result<Json<User>,ServerError>
{
    println!(">>> NEW USER ATTEMPT");
    let hashed_password = md5::compute(&user.password);
    let exists = sqlx::query!(
        "
        SELECT name, password, balance, items
        FROM users
        WHERE name = $1 AND password = $2
        ",
        user.name,
        format!("{:x}", hashed_password)
    ).fetch_one(&state.db).await;

    match exists
    {
        Ok(_) => {
            println!(">>> USER ALREADY EXISTS\n");
            return Err(ServerError::UserExists)
        },
        Err(_) => (),
    }
    
    let result = sqlx::query!(
        "
        INSERT INTO public.users (name, password, balance, items)
        VALUES ($1, $2, $3, $4)
        RETURNING name, password, balance, items
        ",
        user.name,
        format!("{:x}", hashed_password),
        100,
        json!("")
        
    ).fetch_one(&state.db).await;

    
    let user = User
    {
        name: user.name,
        password: format!("{:x}", hashed_password),
        balance: 100,
        items: vec![],
    };
    match result
    {
        Ok(_) => {
            println!(">>> USER CREATED\n");
            Ok(Json(user))
        },
        Err(_) => {
            println!(">>> DATABASE ERROR\n");
            Err(ServerError::DatabaseError)
        },
    }
}

pub async fn delete_user(state: State<Arc<AppState>>, Json(user): Json<UserRequest>) -> Result<Json<UserRequest>,ServerError>
{
    println!(">>> DELETE USER ATTEMPT");
    let hashed_password = md5::compute(&user.password);
    let result = sqlx::query!(
        "
        DELETE FROM public.users
        WHERE name = $1 AND password = $2
        RETURNING name, password, balance, items
        ",
        user.name,
        format!("{:x}", hashed_password)
    ).fetch_one(&state.db).await;
    let user = UserRequest
    {
        name: user.name,
        password: user.password,
    };
    match result
    {
        Ok(_) => {
            println!(">>> USER DELETED\n");
            Ok(Json(user))
        },
        Err(_) => {
            println!(">>> USER NOT FOUND\n");
            Err(ServerError::UserNotFound)
        },
    }
}

use std::{collections::HashMap, str::FromStr, sync::Arc};
use axum::{extract::State, Json};
use serde_json::json;
use crate::{context::Context, error::ServerError, models::{Item, ItemRequest}, AppState};



pub async fn list_items(state: State<Arc<AppState>>,context: Result<Context,ServerError>) -> Result<Json<Vec<(Item,i32)>>,ServerError>
{
    let ctx = context.map_err(|_| ServerError::NotLoggedIn)?;
    let result = sqlx::query!(
        "
        SELECT items
        FROM public.users
        WHERE name = $1
        ",
        ctx.get_name()
    ).fetch_one(&state.db).await;
    match result
    {
        Ok(user) => {
            println!(">>> ITEMS LISTED :: {}\n",ctx.get_name());
            let val: Vec<(Item, i32)> = serde_json::from_value::<Vec<(Item,i32)>>(user.items.clone().unwrap()).unwrap();
            Ok(Json(val))
        },
        Err(_) => {
            println!(">>> USER NOT FOUND\n");
            Err(ServerError::UserNotFound)
        },
    }
}

pub async fn list_store(state: State<Arc<AppState>>,context: Result<Context,ServerError>) -> Result<Json<HashMap<Item,i32>>,ServerError>
{
    let ctx = context.map_err(|_| ServerError::NotLoggedIn)?;
    let result = sqlx::query!(
        "
        SELECT name, price
        FROM public.prices
        "
    ).fetch_all(&state.db).await;
    match result
    {
        Ok(store) => {
            println!(">>> STORE LISTED :: {}\n",ctx.get_name());
            let val: HashMap<Item, i32> = store.iter().map(|item| (Item::from_str(item.name.clone().unwrap().as_str()).unwrap_or(Item::None),item.price.unwrap())).collect();
            Ok(Json(val))
        },
        Err(_) => {
            println!(">>> STORE NOT FOUND\n");
            Err(ServerError::DatabaseError)
        },
    }
}

pub async fn buy_item(state: State<Arc<AppState>>,context: Result<Context,ServerError>,Json(item): Json<ItemRequest>) -> Result<Json<Vec<(Item,i32)>>,ServerError>
{
    let mut ctx = context.map_err(|_| ServerError::NotLoggedIn)?;
    let result = sqlx::query!(
        "
        SELECT balance
        FROM public.users
        WHERE name = $1
        ",
        ctx.get_name()
    ).fetch_one(&state.db).await;
    match result
    {
        Ok(user) => {
            let balance = user.balance.unwrap();
            let item_price = sqlx::query!(
                "
                SELECT price
                FROM public.prices
                WHERE name = $1
                ",
                item.item.to_string()
            ).fetch_one(&state.db).await.unwrap().price.unwrap();
            if balance < item_price*item.number
            {
                println!(">>> NOT ENOUGH FUNDS\n");
                return Err(ServerError::NotEnoughFunds);
            }
            let new_balance = balance - item_price*item.number;
            ctx.add_item(item.item.clone(),item.number);
            let result = sqlx::query!(
                "
                UPDATE public.users
                SET balance = $1 , items = $3
                WHERE name = $2
                RETURNING name, balance, items
                ",
                new_balance,
                ctx.get_name(),
                json!(ctx.get_items())
            ).fetch_one(&state.db).await;
            match result
            {
                Ok(_) => {
                    println!(">>> ITEM BOUGHT :: {}\n",item.item.to_string());
                    Ok(Json(ctx.get_items()))
                },
                Err(_) => {
                    println!(">>> ITEM NOT BOUGHT\n");
                    Err(ServerError::DatabaseError)
                },
            }
        },
        Err(_) => {
            println!(">>> USER NOT FOUND\n");
            Err(ServerError::UserNotFound)
        },
    }
}

pub async fn sell_item(state: State<Arc<AppState>>,context: Result<Context,ServerError>,Json(item): Json<ItemRequest>) -> Result<Json<Vec<(Item,i32)>>,ServerError>
{
    let mut ctx = context.map_err(|_| ServerError::NotLoggedIn)?;
    let result = sqlx::query!(
        "
        SELECT balance
        FROM public.users
        WHERE name = $1
        ",
        ctx.get_name(),
    ).fetch_one(&state.db).await;
    match result
    {
        Ok(user) => {
            let balance = user.balance.unwrap();
            let item_price = sqlx::query!(
                "
                SELECT price
                FROM public.prices
                WHERE name = $1
                ",
                item.item.to_string()
            ).fetch_one(&state.db).await.unwrap().price.unwrap();
            let check = ctx.remove_item(item.item.clone(), item.number);
            match check {
                Ok(num) => {
                    let new_balance = balance + item_price*num;
                    let result = sqlx::query!(
                        "
                        UPDATE public.users
                        SET balance = $1 , items = $3
                        WHERE name = $2
                        RETURNING name, balance, items
                        ",
                        new_balance,
                        ctx.get_name(),
                        json!(ctx.get_items())
                    ).fetch_one(&state.db).await;
                    match result
                    {
                        Ok(_) => {
                            println!(">>> ITEM SOLD :: {}\n",item.item.to_string());
                            Ok(Json(ctx.get_items()))
                        },
                        Err(_) => {
                            println!(">>> ITEM NOT SOLD\n");
                            Err(ServerError::DatabaseError)
                        },
                    }
                }
                Err(_) => {
                    println!(">>> ITEM NOT FOUND\n");
                    return Err(ServerError::OtherError);
                },
            }

        }

        Err(_) => {
            println!(">>> USER NOT FOUND\n");
            Err(ServerError::UserNotFound)
        },
    }
}
    
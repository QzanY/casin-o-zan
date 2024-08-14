use std::sync::Arc;

use axum::{extract::State, response::Html, Json};
use rand::Rng;

use crate::{context::Context, error::ServerError, models::DiceRequest, AppState};

fn random_dice_roll() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

pub async fn dice_game(state: State<Arc<AppState>>, context: Result<Context,ServerError>,Json(request) : Json<DiceRequest>) -> Result<Html<String>,ServerError>
{
    let mut ctx = context.map_err(|_| ServerError::NotLoggedIn)?; 
    if request.bet as i32> *ctx.get_balance()
    {
        return Err(ServerError::NotEnoughFunds);
    }
    let mut nums = vec![];
    let mut sum:u16 = 0;
    for _ in 0..request.number
    {
        let rnd = random_dice_roll();
        nums.push(rnd);
        sum += rnd;
    }
    let status = if sum == request.guess
    {
        ("won",1)
    }
    else
    {
        ("lost",-1)
    };
    let multiplier: i32 = match status.1
    {
        1 => request.number as i32,
        _ => 1,
    };
    let new_balance = *ctx.get_balance() + (request.bet as i32 * status.1 * multiplier);
    match sqlx::query!(
        "
        UPDATE public.users
        SET balance = $1
        WHERE name = $2
        ",
        new_balance,
        ctx.get_name()
    ).execute(&state.db).await
    {
        Ok(_) => ctx.set_balance(new_balance),
        Err(_) => return Err(ServerError::DatabaseError),
    };

    Ok(
        Html(format!(
            "<h1>You {}! </h1>
            <h3>Your guess: {}
            You rolled the dices {} with total {}</h3>
            <h2>Your new balance is: {}</h2>",
            status.0,
            request.guess,
            nums.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
            sum,
            new_balance
        ))
    )

}
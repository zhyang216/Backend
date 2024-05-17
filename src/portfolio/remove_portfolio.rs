use rocket::form::{Form, Strict};
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket_db_pools::{diesel, Connection};
use rocket_db_pools::diesel::prelude::RunQueryDsl;
use ::diesel::ExpressionMethods;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::dsl::sql;
use diesel::sql_types::BigInt;
use diesel::query_dsl::JoinOnDsl;
use rocket::serde::json::Json;

use crate::auth::user_center;
use crate::db_lib::{database, USER_COOKIE_NAME};
use crate::db_lib::schema::{sessions, accounts, portfolios, portfolio_balance};

#[derive(FromForm)]
pub(crate) struct RemovePortfolioInfo<'r> {
    name: &'r str
}

#[post("/remove_portfolio", data = "<remove_portfolio_info>")]
pub(crate) async fn remove_portfolio(
    remove_portfolio_info: Form<Strict<RemovePortfolioInfo<'_>>>, 
    mut db_coon: Connection<database::PgDb>, 
    cookies: &CookieJar<'_>
) -> Result<Status, (Status, &'static str)> {

    // ensure the user is logged in
    if let Some(_) = user_center::get_logged_in_user_id(cookies, &mut db_coon).await {
    } else {
        return Err((Status::BadRequest, "Cannot fetch user id based on session token cookie or cookie crushed."));
    };
    
    // get portfolio's id
    let portfolio_id_result: Result<i32, _> = portfolios::table.filter(portfolios::name.eq(remove_portfolio_info.name))
    .select(portfolios::id)
    .first(&mut db_coon)
    .await;

    let portfolio_id = match portfolio_id_result {
    Ok(id) => id,
    Err(_) => {
        return Err((Status::BadRequest, "The portfolio does not exist"));
    }
    };

    // delete portfolio_balance 
    let portfolio_balance = diesel::delete(portfolio_balance::table.filter(portfolio_balance::portfolio_id.eq(portfolio_id)))
    .execute(&mut db_coon)
    .await;

    // delete portfolio
    let portfolio = diesel::delete(portfolios::table.filter(portfolios::id.eq(portfolio_id)))
    .execute(&mut db_coon)
    .await;

    match portfolio_balance {
        Ok(_) => {
            match portfolio{
                Ok(_) => {
                    return Ok(Status::Ok);
                }
                Err(_) => {
                    return Err((Status::BadRequest, "Can not remove portfolio"));
                }
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err((Status::BadRequest, "Can not remove balance"));
        }
    }  
}
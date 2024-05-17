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
pub(crate) struct AddPortfolioInfo<'r> {
    name: &'r str,
    amount: i32,
    currency_id: i32
}

#[post("/add_portfolio", data = "<add_portfolio_info>")]
pub(crate) async fn add_portfolio(
    add_portfolio_info: Form<Strict<AddPortfolioInfo<'_>>>, 
    mut accounts_db_coon: Connection<database::AccountsDb>,
    cookies: &CookieJar<'_>
) -> Result<Status, (Status, &'static str)> {

    // ensure the user is logged in
    let user_id = if let Some(user_id) = user_center::get_logged_in_user_id(cookies, &mut accounts_db_coon).await {
        user_id
    } else {
        return Err((Status::BadRequest, "Cannot fetch user id based on session token cookie or cookie crushed."));
    };


    // insert the portfolio data into the database
    let portfolio_id = rocket_db_pools::diesel::insert_into(portfolios::table)
    .values((
        portfolios::name.eq(add_portfolio_info.name.to_string()),
        portfolios::trader_account_id.eq(user_id),
        portfolios::portfolio_type.eq(0),
    ))
    .execute(&mut accounts_db_coon).await;

    let portfolio_balance = rocket_db_pools::diesel::insert_into(portfolio_balance::table)
    .values((
        portfolio_balance::quantity.eq(sql::<BigInt>(&add_portfolio_info.amount.to_string())),
        portfolio_balance::currency_id.eq(add_portfolio_info.currency_id)
    ))
    .execute(&mut accounts_db_coon).await;
    
    match portfolio_id {
        Ok(_) => {
            match portfolio_balance{
                Ok(_) => {
                    return Ok(Status::Ok);
                }
                Err(_) => {
                    return Err((Status::BadRequest, "Can not set balance"));
                }
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err((Status::BadRequest, "Portfolio already exists."));
        }
    }
}
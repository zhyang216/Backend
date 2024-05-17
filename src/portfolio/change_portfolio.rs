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
pub(crate) struct ChangePortfolioInfo<'r> {
    name: &'r str,
    amount: i32
}

#[post("/change_portfolio", data = "<change_portfolio_info>")]
pub(crate) async fn change_portfolio(
    change_portfolio_info: Form<Strict<ChangePortfolioInfo<'_>>>, 
    mut db_coon: Connection<database::PgDb>,
    cookies: &CookieJar<'_>
) -> Result<Status, (Status, &'static str)> {
    // ensure the user is logged in
    let user_id = if let Some(user_id) = user_center::get_logged_in_user_id(cookies, &mut db_coon).await {
        user_id
    } else {
        return Err((Status::BadRequest, "Cannot fetch user id based on session token cookie or cookie crushed."));
    };

    // get portfolio's id
    let portfolio_id_result: Result<i32, _> = portfolios::table
        .filter(portfolios::name.eq(&change_portfolio_info.name))
        .select(portfolios::id)
        .first(&mut db_coon)
        .await;

    let portfolio_id = match portfolio_id_result {
        Ok(id) => id,
        Err(_) => {
            return Err((Status::BadRequest, "The portfolio does not exist"));
        }
    };

    // update portfolio_balance 
    let update_result = diesel::update(portfolio_balance::table.filter(portfolio_balance::portfolio_id.eq(portfolio_id)))
        .set(portfolio_balance::quantity.eq(change_portfolio_info.amount as i64))
        .execute(&mut db_coon)
        .await;

    match update_result {
        Ok(updated_records) => {
            if updated_records == 0 {
                // not find matched portfolio
                return Err((Status::BadRequest, "Portfolio balance not found"));
            } else {
                return Ok(Status::Ok);
            }
        }
        Err(_) => {
            return Err((Status::InternalServerError, "Failed to update portfolio balance"));
        }
    }
}

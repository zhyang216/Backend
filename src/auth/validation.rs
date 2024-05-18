use ::diesel::ExpressionMethods;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use rocket_db_pools::diesel::prelude::RunQueryDsl;

use crate::db_lib::database;
use crate::db_lib::schema::sessions;
use crate::db_lib::session::SessionToken;
use crate::db_lib::USER_COOKIE_NAME;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};
use rocket_db_pools::Connection;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserAuth {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let mut db_conn = req.guard::<Connection<database::PgDb>>().await.unwrap();
        let cookies = req.cookies();
        // get the session token from the client(cookie)
        let fetch_cookie = cookies
            .get_private(USER_COOKIE_NAME)
            .and_then(|cookie| cookie.value().parse::<String>().ok());

        let session_token = if let Some(cookie) = fetch_cookie {
            SessionToken::to_token(cookie)
        } else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        // get the user id corresponding to the session token from the database
        let fetch_user_id = sessions::table
            .select(sessions::user_id)
            .filter(sessions::session_token.eq(session_token.into_database_value()))
            .first::<i32>(&mut db_conn)
            .await;

        if let Ok(user_id) = fetch_user_id {
            return Outcome::Success(UserAuth { user_id: user_id });
        } else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
    }
}

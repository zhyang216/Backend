use std::env;

use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use dotenv::dotenv;
use rand_core::{OsRng, RngCore};
use rocket::State;
use rocket::form::{Form, Strict};
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar, Status};
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::RunQueryDsl;
use ::diesel::ExpressionMethods;
use pbkdf2::password_hash::PasswordHash;
use pbkdf2::{password_hash::PasswordVerifier, Pbkdf2,};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use chrono::{Duration, Utc};
use urlencoding::encode;

use crate::schema::users;
use crate::session::new_session;
use crate::user_center::get_logged_in_user_id;
use crate::USER_COOKIE_NAME;
use crate::{database, RAND};

#[derive(FromForm)]
pub(crate) struct LoginInfo<'r> {
    user_name: &'r str,
    user_password: &'r str
}

// If login successfully, a session token will be saved in both server(database) and the client(cookie), finally redirect to index page
// Otherwise, Status::Badrequest is returned (not fancy at all)
#[post("/login", data = "<login_info>")]
pub(crate) async fn login(
    login_info: Form<Strict<LoginInfo<'_>>>,
    mut accounts_db_coon: Connection<database::AccountsDb>, 
    cookies: &CookieJar<'_>,
    random: &State<RAND>
) -> Result<Redirect, (Status, &'static str)> {
    
    // query the id and (hashed)password in the database according to the username
    let login_result = users::table
        .select((users::id, users::password))
        .filter(users::username.eq(login_info.user_name.to_string()))
        .first::<(i32, String)>(&mut accounts_db_coon).await;

    // If query fails, return badquest
    let (user_id, hashed_password) = if let Ok(login_result_ok) = login_result {
        login_result_ok
    } else {
        return Err((Status::BadRequest, "Login fails. Probably wrong username or password."));
    };

    // If (hashed)password doesn't match, return badrequest
    if let Err(_err) = Pbkdf2.verify_password(login_info.user_password.as_bytes(), &PasswordHash::new(&hashed_password).unwrap()) {
        return Err((Status::BadRequest, "Wrong password."));
    }

    // Generate a session key. Save it in both the server(database) and the client(cookie).
    let token = new_session(random.random.clone(), user_id, &mut accounts_db_coon).await;
    match token {
        Ok(token) => {
            let cookie_value = token.into_cookie_value();
            cookies.add_private(Cookie::build((USER_COOKIE_NAME, cookie_value))); // default expire time: one week from now
            // Redirect to index page
            return Ok(Redirect::to(uri!("/index")));
        },
        Err(session_err) => {
            return Err(session_err);
        }
    }
}


#[derive(FromForm)]
pub(crate) struct ForgetPasswordInfo<'r> {
    user_name: &'r str
}

// this function doesn't work since send_reset_password_email doesn't work
#[post("/login/forget_password", data = "<forget_password_info>")]
pub(crate) async fn forget_password(
    forget_password_info: Form<Strict<ForgetPasswordInfo<'_>>>,
    mut accounts_db_coon: Connection<database::AccountsDb>, 
    cookies: &CookieJar<'_>
) -> Result<(Status, &'static str), (Status, &'static str)> {

    if let Some(_) = get_logged_in_user_id(cookies, &mut accounts_db_coon).await {
        return Err((Status::BadRequest, "Already Logged in."));
    }

    println!("{}", forget_password_info.user_name);
    let fetch_user_email = users::table
        .select(users::email)
        .filter(users::username.eq(forget_password_info.user_name.to_string()))
        .first::<String>(&mut accounts_db_coon).await;

    let user_email = if let Ok(user_email) = fetch_user_email {
        user_email
    } else {
        return Err((Status::BadRequest, "User email not found"));
    };

    let reset_token = OsRng.next_u64();
    let send_email = send_reset_password_email(forget_password_info.user_name, &user_email, &reset_token).await;

    match send_email {
        Ok(_) => Ok((Status::Accepted, "The email is successfully sent")),
        Err(_) => Err((Status::BadRequest, "Fail to send the email."))
    }
}


// this function doesn't work
pub(crate) async fn send_reset_password_email(
    user_name: &str,
    user_email: &str,
    reset_token: &u64
) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    dotenv().ok();

    let smtp_key: &str = "xsmtpsib-9d8ddef88e873542e39024400bc521ce08ee8e6b2973217ca225fb8b8247de03-ZNSnbB5smgvXy6Yc";
    let from_email: &str = "testgdscmail@gmail.com";
    let to_email: &str = &user_email;
    let host: &str = "smtp-relay.sendinblue.com";
    let expiration_time = Utc::now() + Duration::minutes(5);
    let reset_link = format!("{}/login/forget_password/{}/{}/{}", env::var("DOMAIN").unwrap(), user_name, reset_token, encode(&expiration_time.to_rfc3339()));
    
    let email: Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Reset your password")
        .body(format!(
            "Please click the following link to reset your password:\nLink:{}\nThe link will expired in 5 minutes.",
            reset_link))
        .unwrap();

    let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

    // Open a remote connection to gmail
    let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    return mailer.send(&email);
}
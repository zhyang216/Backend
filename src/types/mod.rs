use rocket::serde::json::Json;
use rocket::response::{status, Response};
use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData {
    pub status: String,
    pub data: Option<Vec<String>>,
	pub len: Option<i32>,
	pub message: Option<String>,
}
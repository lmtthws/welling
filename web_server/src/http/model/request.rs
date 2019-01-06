use std;
use uri::*;
use crate::response::StatusCode;

//TODO: provide accessors and a constructor 
#[derive(PartialEq, Debug)]
pub struct StartLine {
	pub method: AllowedMethod,
	pub uri: Uri,
	pub major_version: usize,
	pub minor_version: usize
}

#[derive(PartialEq, Debug)]
pub enum AllowedMethod {
	GET,
	POST
}

impl std::fmt::Display for AllowedMethod {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			AllowedMethod::GET => write!(f, "GET"),
			AllowedMethod::POST => write!(f, "POST")
		}
	}
}

impl AllowedMethod {
	pub fn from_utf8(raw_method: String) -> Result<AllowedMethod, (StatusCode, String)> {

		if raw_method.len() > 4 {
			return Err((StatusCode::not_implemented(), String::from("Method name too long")));
		}

		match raw_method.as_ref() {
			"GET" =>  return Ok(AllowedMethod::GET),
			"POST" => return Ok(AllowedMethod::POST),
			_ => return Err((StatusCode::internal_server_error(), String::from("Method not supported")))
		}
	}
}
extern crate uri;

use std;
use uri::*;

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
	pub fn from_utf8(raw_method: String) -> Result<AllowedMethod, (usize, String)> {

		if raw_method.len() > 4 {
			return Result::Err((514, String::from("Method name too long")));
		}

		match raw_method.as_ref() {
			"GET" =>  return Result::Ok(AllowedMethod::GET),
			"POST" => return Result::Ok(AllowedMethod::POST),
			_ => return Result::Err((500, String::from("Method not supported")))
		}
	}
}
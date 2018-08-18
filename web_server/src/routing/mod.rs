use uri::Uri;

use http::model::*;

//consider integrating constructor with Routable or replacing it
macro_rules! get {
	($u:expr) => { Routable::get(&$u)};
}

macro_rules! post {
	($u:expr) => {Routable::post(&$u)};
}
//TODO: separate http version from the StartLine



pub trait Routable {
	fn get(&self) -> StartLine;
	fn post(&self) -> StartLine;
}

impl<'a> Routable for &'a str {
	fn get(&self) -> StartLine {
		Routable::get(&self.to_string())
	}

	fn post(&self) -> StartLine {
		Routable::post(&self.to_string())
	}
}

impl Routable for String {
	fn get(&self) -> StartLine {
		StartLine {
			method: AllowedMethod::GET,
			uri: Uri::new(self),
			major_version: 1,
			minor_version: 1
		}
	}

	fn post(&self) -> StartLine {
		StartLine {
			method: AllowedMethod::POST,
			uri: Uri::new(self),
			major_version: 1,
			minor_version: 1
		}
	}
}
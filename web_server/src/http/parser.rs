<<<<<<< Updated upstream
extern crate uri;

use std;
use std::net::TcpStream;
use std::io::Read;
use uri::*;
use http::model::*;
use std::time::Duration;

const MAX_START_LEN: usize = 8192;

pub struct HttpRequestParser {
	buffer: Vec<u8>
}

//TODO: create context which owns the stream and provides response capabilities
// 		switch the parser to interact with a mutable &TcpStream

impl HttpRequestParser {
	pub fn new() -> HttpRequestParser{
		HttpRequestParser {
			buffer: Vec::new()
		}
	}

	pub fn get_start_line<'a>(&mut self, stream: &mut TcpStream) -> std::result::Result<StartLine, (usize, String)> {

		let mut line: Vec<u8>  = Vec::new();
		let mut line_terminated = false;

		//TODO: handle empty reads from the stream

		while !line_terminated {
			if line.len() > MAX_START_LEN {
				return Result::Err((414, String::from("Request target longer than max allowed length")))
			}
			let local_buf: &mut [u8] = &mut [0_u8; 512];
			stream.read(local_buf).unwrap();

			let local_buf = local_buf.to_vec();	
			println!("{}", String::from_utf8(local_buf.clone()).expect("Failed to read buffer into string"));

			for ix in 0..510 {
				let fc = local_buf[ix];
				let sc = local_buf[ix+1];
				if let (b'\r', b'\n') = (fc, sc) {
					line_terminated = true;
					if ix < 510 { 
						self.buffer.extend(local_buf[ix+2..].iter().cloned());
					}
					break;
				} else {
					line.push(local_buf[ix]);
				}
			}
		}

		let mut method_term_ix = line.len();
		for ix in 0..line.len() {
			if b' ' == *(line.get(ix).unwrap()) {
				method_term_ix = ix;
				break;
			}
		}

		let mut vers_start_ix = method_term_ix;
		for ix in (method_term_ix..line.len() - 1).rev() {
			if b' ' == *(line.get(ix).unwrap()) {
				vers_start_ix = ix;
				break;
			}
		}

		let version = String::from_utf8(line.get(vers_start_ix+1..line.len()).unwrap().to_vec()).unwrap();
		if version.len() != 8 || !version.starts_with("HTTP/") {
			return Err((500, String::from("Invalid HTTP Version value"))) //Check on what IIS does
		}

		let method = String::from_utf8(line.get(0..method_term_ix).unwrap().to_vec()).unwrap();
		let method = AllowedMethod::from_utf8(method)?;

		let major_version = usize::from_str_radix(version.get(5..6).unwrap(), 10).unwrap();
		let minor_version = usize::from_str_radix(version.get(7..8).unwrap(), 10).unwrap();


		let uri = String::from_utf8(line.get(method_term_ix+1..vers_start_ix).unwrap().to_vec()).unwrap();
		let uri = Uri::new(&uri);

		Ok(StartLine {
			method,
			uri,
			major_version,
			minor_version
		})	//501 returned if the method is longer than any supported
		//414 returned if request-target is longer than max allowed uri length
		//  recommended that recipients support request-lines of 8000 octects 
		//		- 4000 chars in UTF-16 and max 8000 in utf-8 and 2000 in utf-32
	}

	pub fn get_request_body(&mut self, stream: &mut TcpStream) {
		let local_buf: &mut [u8] = &mut [0_u8; 512];

		stream.set_read_timeout(Some(Duration::new(1,0))).expect("This should only fail if I passed in zero");
		match stream.read(local_buf)
		{
			_ => (),
		}

		let local_buf = local_buf.to_vec();	
		println!("{}", String::from_utf8(local_buf.clone()).expect("Failed to read buffer into string"));
	}
}

=======
extern crate uri;

use std;
use std::net::TcpStream;
use std::io::Read;
use uri::*;
use http::request::StartLine;
use http::request::AllowedMethod;
use std::time::Duration;

const MAX_START_LEN: usize = 8192;

pub struct HttpRequestParser {
	buffer: Vec<u8>
}

//TODO: create context which owns the stream and provides response capabilities
// 		switch the parser to interact with a mutable &TcpStream

impl HttpRequestParser {
	pub fn new() -> HttpRequestParser{
		HttpRequestParser {
			buffer: Vec::new()
		}
	}

	pub fn get_start_line<'a>(&mut self, stream: &mut TcpStream) -> std::result::Result<StartLine, (usize, String)> {

		let mut line: Vec<u8>  = Vec::new();
		let mut line_terminated = false;

		//TODO: handle empty reads from the stream

		while !line_terminated {
			if line.len() > MAX_START_LEN {
				return Result::Err((414, String::from("Request target longer than max allowed length")))
			}
			let local_buf: &mut [u8] = &mut [0_u8; 512];
			stream.read(local_buf).unwrap();

			let local_buf = local_buf.to_vec();	
			println!("{}", String::from_utf8(local_buf.clone()).expect("Failed to read buffer into string"));

			for ix in 0..510 {
				let fc = local_buf[ix];
				let sc = local_buf[ix+1];
				if let (b'\r', b'\n') = (fc, sc) {
					line_terminated = true;
					if ix < 510 { 
						self.buffer.extend(local_buf[ix+2..].iter().cloned());
					}
					break;
				} else {
					line.push(local_buf[ix]);
				}
			}
		}

		let mut method_term_ix = line.len();
		for ix in 0..line.len() {
			if b' ' == *(line.get(ix).unwrap()) {
				method_term_ix = ix;
				break;
			}
		}

		let mut vers_start_ix = method_term_ix;
		for ix in (method_term_ix..line.len() - 1).rev() {
			if b' ' == *(line.get(ix).unwrap()) {
				vers_start_ix = ix;
				break;
			}
		}

		let version = String::from_utf8(line.get(vers_start_ix+1..line.len()).unwrap().to_vec()).unwrap();
		if version.len() != 8 || !version.starts_with("HTTP/") {
			return Err((500, String::from("Invalid HTTP Version value"))) //Check on what IIS does
		}

		let method = String::from_utf8(line.get(0..method_term_ix).unwrap().to_vec()).unwrap();
		let method = AllowedMethod::from_utf8(method)?;

		let major_version = usize::from_str_radix(version.get(5..6).unwrap(), 10).unwrap();
		let minor_version = usize::from_str_radix(version.get(7..8).unwrap(), 10).unwrap();


		let uri = String::from_utf8(line.get(method_term_ix+1..vers_start_ix).unwrap().to_vec()).unwrap();
		let uri = Uri::new(&uri);

		Ok(StartLine {
			method,
			uri,
			major_version,
			minor_version
		})	//501 returned if the method is longer than any supported
		//414 returned if request-target is longer than max allowed uri length
		//  recommended that recipients support request-lines of 8000 octects 
		//		- 4000 chars in UTF-16 and max 8000 in utf-8 and 2000 in utf-32
	}

	pub fn get_request_body(&mut self, stream: &mut TcpStream) {
		let local_buf: &mut [u8] = &mut [0_u8; 512];

		stream.set_read_timeout(Some(Duration::new(1,0))).expect("This should only fail if I passed in zero");
		match stream.read(local_buf)
		{
			_ => (),
		}

		let local_buf = local_buf.to_vec();	
		println!("{}", String::from_utf8(local_buf.clone()).expect("Failed to read buffer into string"));
	}
}

>>>>>>> Stashed changes

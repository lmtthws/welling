extern crate uri;

use std;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Read};
use uri::*;
use http::model::*;
use std::time::Duration;

mod grammar;

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

	pub fn get_start_line(&mut self, stream: &mut TcpStream) -> std::result::Result<StartLine, StatusLine> {

		let mut line: Vec<u8>  = Vec::new();
		let mut line_terminated = false;

		//TODO: handle empty reads from the stream

		while !line_terminated {
			if line.len() > MAX_START_LEN {
				return Err(StatusLine::init(StatusCode::uri_too_long(), String::from("Request target longer than max allowed length")))
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
			return Err(StatusLine::init(StatusCode::internal_server_error(), String::from("Invalid HTTP Version value"))) //Check on what IIS does
		}

		let method = String::from_utf8(line.get(0..method_term_ix).unwrap().to_vec()).unwrap();
		let method = match AllowedMethod::from_utf8(method) {
			Ok(a) => a,
			Err((c,s)) => return Err(StatusLine::init(c,s))
		};

		let major_version = usize::from_str_radix(version.get(5..6).unwrap(), 10).unwrap();
		let minor_version = usize::from_str_radix(version.get(7..8).unwrap(), 10).unwrap();

		let uri = String::from_utf8(line.get(method_term_ix+1..vers_start_ix).unwrap().to_vec()).unwrap();
		let uri = Uri::new(&uri);

		Ok(StartLine {
			method,
			uri,
			major_version,
			minor_version
		})	
		//501 returned if the method is longer than any supported
		//414 returned if request-target is longer than max allowed uri length
		//  recommended that recipients support request-lines of 8000 octects 
		//		- 4000 chars in UTF-16 and max 8000 in utf-8 and 2000 in utf-32
	}

	pub fn get_request_headers<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<HeaderCollection,StatusLine> {
		const MAX_HEADER_LEN: usize = 8000;
		
		let mut headers = HeaderCollection::init_empty();

		let mut header_buf: Vec<u8> = Vec::new();
		let mut name_end: usize = MAX_HEADER_LEN + 1;
		while let Ok(3...MAX_HEADER_LEN) = reader.read_until(b'\n', &mut header_buf) {
			for ix in 0..header_buf.len() - 1 {
				let parse_char = *header_buf.get(ix).unwrap();

				if b':' == parse_char {
					name_end = ix;
					break;
				} else if parse_char == b' ' || parse_char == b'\t' {
					if ix == 0 {
						return Err(StatusLine::init(StatusCode::bad_request(), String::from("line folding is obsolete and not accepted")))
					} else {
						return Err(StatusLine::init(StatusCode::bad_request(), String::from("invalid whitespace in header field name")))
					}
				}
			}

			if name_end == MAX_HEADER_LEN + 1 {
				return Err(StatusLine::init(StatusCode::bad_request(), String::from("header field too long")))
			}

			let (header_name, header_val) = header_buf.split_at(name_end);
			
			let field_name = match String::from_utf8(header_name.to_vec()) {
				Ok(n) => n,
				Err(_) => return Err(StatusLine::init(StatusCode::bad_request(), String::from("invalid header field name")))
			};

			let mut header = headers.add_or_get_mut(&field_name);

			let mut value = Vec::new();
			for c in header_val {
				if *c == b'"' {

				}
				else if *c == b' ' || *c == b'\t' {
					let val = match String::from_utf8(value) {
						Ok(v) => v,
						Err(_) => return Err(StatusLine::init(StatusCode::bad_request(), String::from("invalid header field value")))
					};
					header.add(val);
					value = Vec::new();
				}
				
			}
			
		}

		Ok(headers)
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


extern crate uri;

use std;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Read};
use uri::*;
use http::model::*;
use std::time::Duration;

mod grammar;
use self::grammar::*;

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
				
				if parse_char.is_header_field_delim() {
					name_end = ix;
					break;
				} else if parse_char.is_whitespace_char() {
					if ix == 0 {
						return Err(StatusLine::init(StatusCode::bad_request(), String::from("line folding is obsolete and not accepted")))
					} else {
						return Err(StatusLine::init(StatusCode::bad_request(), String::from("invalid whitespace in header field name")))
					}
				} else if !parse_char.is_token_char() {
					return Err(StatusLine::init(StatusCode::bad_request(), String::from("header field name contained non-token character")))
				}
			}

			if name_end == MAX_HEADER_LEN + 1 {
				return Err(StatusLine::init(StatusCode::bad_request(), String::from("header field too long")))
			}

			let (header_name, header_val) = header_buf.split_at(name_end);
			
			if header_val.len() == 0 {
				return Err(StatusLine::init(StatusCode::bad_request(), String::from("value was not provided for a header")))
			}

			let header_name = String::from_utf8(header_name.to_vec()).unwrap(); //this is safe, since we validated that every byte was a valid token character (ASCII subset)
			let mut header = headers.add_or_get_mut(&header_name);
			self.read_in_header_val(header_val, header)?
		}

		Ok(headers)
	}

	pub(self) fn read_in_header_val(&self, value_chars: &[u8], header: &mut HttpHeader) -> Result<(),StatusLine> {
		let mut escaping: bool = false;
		let mut is_quoting: bool = false;
		let mut comment_depth: u8 = 0;
		let mut value = Vec::new();

		for c in value_chars {
			//escape check must come first, as it is valid within quotes or comments
			if escaping {
				value.push(*c);
				escaping = false;
			} else if c.is_escape_char() {
				escaping = true;
			} 
			
			//next quoting
			else if is_quoting {
				if c.is_dquote_char() {
					is_quoting = false;
					header.add(String::from_utf8(value).unwrap());
					value = Vec::new()
				} else if c.is_quoted_text() {
					value.push(*c);
				} else {
					return Err(StatusLine::init(StatusCode::bad_request(), String::from("quoted header value contained invalid character")))
				}
			} else if c.is_dquote_char() && comment_depth == 0 {
				is_quoting = true;
			}
			  
			//then comments
			else if c.is_field_comment_start() {
				println!("Depth at comment start: {}", comment_depth);
				value.push(*c);
				comment_depth += 1;	
			} else if c.is_field_comment_end() {
				comment_depth -= 1;
				println!("Depth after comment end: {}", comment_depth);
				value.push(*c);
				
				if comment_depth == 0 {
					header.add(String::from_utf8(value).unwrap());
					value = Vec::new();	
				}				
			} else if comment_depth > 0 {
				if c.is_comment_text() {
					value.push(*c)
				} else {
					return Err(StatusLine::init(StatusCode::bad_request(), String::from("comment contained invalid character")))
				}
			}

			//everything else
			else if c.is_whitespace_char() {
				header.add(String::from_utf8(value).unwrap());
				value = Vec::new();
			} else if c.is_token_char() {
				value.push(*c);
			} else {
				return Err(StatusLine::init(StatusCode::bad_request(), String::from("field value contained invalid character")))
			}
		} 

		if is_quoting || escaping || comment_depth > 0 {
			return Err(StatusLine::init(StatusCode::bad_request(), String::from("comment contained unclosed quote, comment, or escape sequence")))
		}

		Ok(())
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

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn read_in_header_val__preceding_white_space__value_is_trimmed() {
		let header_string = String::from("   test test2  test3    ");
		let mut header = HttpHeader::init("ignored");

		let parser = HttpRequestParser::new();
		parser.read_in_header_val(header_string.as_ref(), &mut header).unwrap();

		assert_eq!(header.len(), 3);
		assert_eq!(header.get(0).unwrap(), "test");
		assert_eq!(header.get(1).unwrap(), "test2");
		assert_eq!(header.get(2).unwrap(), "test3");
	}

	#[test]
	fn read_in_header_val__comment__value_is_comment() {
		let header_string = String::from(" test (this is a comment(with a nested \"comment\")) ");
		let mut header = HttpHeader::init("ignored");

		let parser = HttpRequestParser::new();
		parser.read_in_header_val(header_string.as_ref(), &mut header).unwrap();

		println!("{:?}", header.get_all());

		assert_eq!(header.len(), 2);

		assert_eq!(header.get(0).unwrap(), "test");
		assert_eq!(header.get(1).unwrap(), "(this is a comment(with a nested \"comment\"))");
	}

	#[test]
	fn read_in_header_val__quoted__value_is_quoted() {
		let header_string = String::from(" test \"this is a quoted string (with a parenthetical)\" test2 ");
		let mut header = HttpHeader::init("ignored");

		let parser = HttpRequestParser::new();
		parser.read_in_header_val(header_string.as_ref(), &mut header).unwrap();

		println!("{:?}", header.get_all());

		assert_eq!(header.len(), 2);

		assert_eq!(header.get(0).unwrap(), "test");
		assert_eq!(header.get(1).unwrap(), "this is a quoted string (with a parenthetical)");
		assert_eq!(header.get(1).unwrap(), "test2");
	}
}


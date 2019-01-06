
use crate::http::parser::HttpRequestParser;
use crate::http::model::*;
use std::net::TcpStream;
use std::io::Write;

//TODO: use a std::io::BufReader


pub struct HttpContext {
    stream: TcpStream,
    parser: HttpRequestParser
}

impl HttpContext {
    pub fn from_stream(stream: TcpStream) ->HttpContext {
        HttpContext {
            stream: stream,
            parser: HttpRequestParser::new()
        }
    }

    pub(crate) fn get_start_line(&mut self) -> Result<StartLine, StatusLine> {
        self.parser.get_start_line(&mut self.stream)
    }

    pub(crate) fn flush_request(&mut self) {
        self.parser.get_request_body(&mut self.stream)
    }

    pub(crate) fn send_response(&mut self, response: String) {
			self.stream.write(response.as_bytes()).unwrap();
			self.stream.flush().unwrap();
	}

}
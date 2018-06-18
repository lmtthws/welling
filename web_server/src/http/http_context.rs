
use http::parser::HttpRequestParser;
use http::request::StartLine;
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

    pub fn get_start_line(&mut self) -> Result<StartLine, (usize,String)> {
        self.parser.get_start_line(&mut self.stream)
    }

    pub fn flush_request(&mut self) {
        self.parser.get_request_body(&mut self.stream)
    }

    pub fn send_response(&mut self, response: String) {
			self.stream.write(response.as_bytes()).unwrap();
			self.stream.flush().unwrap();
	}

}
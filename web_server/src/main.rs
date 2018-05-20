use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
extern crate thread_pool;

use std::fs::File;
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;
use std::process;


fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let _pool = ThreadPool::new(4).unwrap_or_else(|err| {
		eprintln!("{}", err.message);
		process::exit(1);
	});

	for stream in listener.incoming(){
		let stream = stream.unwrap();


		handle_connection(stream);
	}
}


fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();

	let get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";
	
	let (status, content) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", &"Index.html")
	} else if buffer.starts_with(sleep) {
		thread::sleep(Duration::from_secs(5));
		("HTTP/1.1 200 OK\r\n\r\n", &"Index.html")
	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", &"Error_404.html")
	};

	let response = read_static_response(content).unwrap();
	let response = format!("{}{}",status,response);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
	
}

fn read_static_response(response_name: &str) -> std::io::Result<String> {
	let mut file = File::open(format!("./views/{}", response_name))?;

	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	Ok(contents)
}
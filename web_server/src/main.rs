extern crate thread_pool;
extern crate uri;

use std::fs::File;
use thread_pool::ThreadPool;
use std::process;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

#[macro_use]
mod routing;
mod http;

use routing::Routable;

use http::http_context::HttpContext;
use http::model::*;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4).unwrap_or_else(|err| {
		eprintln!("{}", err.message);
		process::exit(1);
	});

	for stream in listener.incoming(){
		let stream = stream.unwrap();

		pool.execute(|| { handle_connection(stream)});
	}
}

//Note: for http client, need to send as 1.0 instead of 1.1 until server responds with 1.1
//		send the highest major version with which server is conformany, and highest minor version client recognizes and is conformant
// if an intermediary in a pipeline (except tunnel) the forwarded hppt version must be the intermediary's capabilities

// server responds with highest version it is compliant with major vers <= client vers
//	505 if major version is not supported

//if implementing a client, we'll have to do name resolution with DNS to get the IP, maybe...
// the client handles the resolution of the uri to the authority and opening the TCP connection to the port

// note: http is a URI scheme indicating tcp-based communitcation 
// HTTP is the communication protocol which is independent of transport

//https is not reflected in an http message, just that the connection is TCP secured using TLS and 
// that the connection must be secured prior to the first HTTP message being sent

//only scheme and host are case-insenstive - everything else should be sensitive
// urls needs to be %decoded

//parsing must be done using an US-ASCII superset encoding



//we can derive a struct from a [derive(routeTemplate)] or similar
// if the struct cotains unused fields, we can use them in our code generation...
// the method bodies can then specify that Struct as their initial parameter

// I also think you might be able to do some crazy stuff with build.rs, like generate source files which are then compiled and brought into the target dir

//be sure to track fire extinguisher and fire detector batteries
// really, should be able to define and track all sorts of maintenance tasks

//IDEA: have build.rs generate route files in an output directory
// route files used to initialize web server on startup for allowed verb/string combos
// would still need to have handlers in there somehow...

//TODO: parse the headers to get content length
// only read from the stream if post body is longer than what is read so far
//TODO: handle different request content types

//manage the parse via a struct

fn handle_connection(stream: TcpStream) {

	let start_line: StartLine;
	let _buf: Vec<u8>;
	let mut context = HttpContext::from_stream(stream);
	match context.get_start_line() {
		Ok(s) => start_line = s,
		Err(_) => {
			let response = format!("{}{}",500,"Unable to parse starting line");
			context.send_response(response);
			return;
		}
	};


	if let AllowedMethod::GET = start_line.method {
		println!("Parse result - start");
		println!("{} {} HTTP/{}.{}", start_line.method, start_line.uri, start_line.major_version, start_line.minor_version);
	}
	
	let get = get!("/");
	let sleep = get!("/sleep");
	let test = post!("/test/post");

	context.flush_request();	
	let response: String;

	match start_line.method {
		AllowedMethod::GET => {
			let (status, content): (&str, String) = if start_line == get {
				("HTTP/1.1 200 OK\r\n\r\n", "./views/Index.html".to_string())
			} else if start_line == sleep {
				thread::sleep(Duration::from_secs(5));
				("HTTP/1.1 200 OK\r\n\r\n", "./views/Index.html".to_string())
			} else if start_line.uri.path.path_components.starts_with(&[String::from("scripts")]) {
				println!("Script request received: {}", start_line.uri);
				let script_path: String = format!("./{}", start_line.uri.path.path_components.join("/"));
				("HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n", script_path)		
			} else {
				("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Error_404.html".to_string())
			};

			let response_temp = read_static_content(&content).unwrap();
			response = format!("{}{}",status,response_temp);
		},
	 	AllowedMethod::POST => {
	 		println!("Got a post");
 		 		
	 		if start_line == test {
	 			response = String::from("HTTP/1.1 200 OK\r\n\
	 									Content-Type: application/json; charset=UTF-8\r\n\
	 									\r\n\
	 									{\"test\": \"POST successful\" }");
	 		} else {
	 			response = String::from("HTTP/1.1 401 Unauthorized\r\n\r\n");
	 		}
	 	} 
	}


	println!("response returned: {}", response);
	context.send_response(response);
}

fn read_static_content(response_path: &str) -> std::io::Result<String> {
	let mut file = File::open(response_path)?;

	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	Ok(contents)
}
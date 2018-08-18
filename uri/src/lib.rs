use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::vec::IntoIter;
use std::fmt::{Formatter, Display, Result};


#[derive(PartialEq,Debug)]
pub struct Uri {
	pub scheme: Option<String>,
	pub path: HierarchicalPart,
	pub query: Option<String>,
	pub fragment: Option<String>
}

//todo: provide constuctors (via macro? - no overloads)
// switch pub fields to private and provide accessors

enum UriComponent {
	Scheme,
	Path,
	Query,
	Fragment,
	Start
}

impl Uri {

	pub fn new(raw_uri: &str) -> Uri {
		let b = &mut Vec::new();
		let p = &mut Vec::new();

		let raw_uri_bytes = raw_uri.as_bytes();
		let mut cur_comp = UriComponent::Start;
	
		let mut scheme: Option<String> = None;
		let mut path = String::new();
		let mut query: Option<String> = None;
		let mut fragment : Option<String> = None;

		//todo: REMOVE VEC! and just use slices in array

		for ix in 0..raw_uri_bytes.len() {
			let c = raw_uri_bytes[ix];
			match cur_comp {
				UriComponent::Start => {
					match c {
						b'/' => cur_comp = UriComponent::Path,
						b'A'...b'Z' | b'a'...b'z' => {
							cur_comp = UriComponent::Scheme;
							b.push(c);
						},
						_ => panic!("Initial character invalid")
					}
				},
				UriComponent::Scheme => {
					match c {
						b':' => { 
							cur_comp = UriComponent::Path;
							scheme = Some(String::from_utf8(b.clone()).unwrap());
							b.clear()
						},
						b'a'...b'z' 
						| b'A'...b'Z' 
						| b'0'...b'9'  
						| b'.' 
						| b'+' 
						| b'-' => b.push(c),
						_ => panic!("scheme contained invalid character!")
					}
				},
				UriComponent::Path => {
					match c {
						b'?' => {
							cur_comp = UriComponent::Query;
							path = String::from_utf8(p.clone()).unwrap();
							p.clear();
						},
						b'#' => {
							cur_comp = UriComponent::Fragment;
							path = String::from_utf8(p.clone()).unwrap();
							p.clear();
						}
						_ => p.push(c)
					}
				},
				UriComponent::Query => {
					match c {
						b'#' => {
							cur_comp = UriComponent::Fragment;
							query = Some(String::from_utf8(p.clone()).unwrap());
							p.clear();
						},
						_ => p.push(c)
					}
				},
				UriComponent::Fragment => {
					p.push(c);
				}
			}
		}

		match cur_comp {
			UriComponent::Start => (),
			UriComponent::Scheme => scheme = Some(String::from_utf8(b.clone()).unwrap()),
			UriComponent::Path => path = String::from_utf8(p.clone()).unwrap(),
			UriComponent::Query => query = Some(String::from_utf8(p.clone()).unwrap()),
			UriComponent::Fragment => fragment = Some(String::from_utf8(p.clone()).unwrap())
		}	
		
		let path = parse_path(path);	

		return Uri {
			scheme,
			path,
			query,
			fragment
		}
	}

	//TODO: implement read-only style mutators...
}

impl Display for Uri {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut uri: String;

		match self.scheme {
			Some(ref s) => uri = s.to_string(),
			None => uri = String::new()
		}

		uri.push_str(&self.path.to_string());

		if let Some(ref q) = self.query {
			uri.push('?');
			uri.push_str(&q.to_string());
		}

		if let Some(ref f) = self.fragment {
			uri.push('#');
			uri.push_str(&f.to_string());
		}

		write!(f, "{}", uri)
	}
}

impl ToSocketAddrs for Uri {
	type Iter = IntoIter<SocketAddr>;

	fn to_socket_addrs(&self) -> std::io::Result<IntoIter<SocketAddr>> {
		if let Some(ref host_and_port) = self.path.authority {
			Ok(host_and_port.to_socket_addrs()?)
		} else {
			Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "URI does not contain authority information"))
		}
	}
}


#[derive(Debug,PartialEq)]
pub struct HierarchicalPart {
	pub authority: Option<Authority>,
	pub path_components: Vec<String>
}


impl Display for HierarchicalPart {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut authority: String;
		match self.authority {
			Some(ref a) => authority = a.to_string(),
			None => authority = String::new()
		}

		authority.push('/');
		authority.push_str(&self.path_components.join("/"));
		
		write!(f, "{}", authority)
	}
}

#[derive(Debug,PartialEq)]
pub struct Authority {
	pub userinfo: Option<String>,
	pub host: Option<String>,
	pub port: Option<usize>
}

impl Display for Authority {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut auth: String;

		match self.userinfo {
			Some(ref info) => {
				auth = info.to_string();
				auth.push('@');
			},
			None => auth = String::new(),
		}

		if let Some(ref h) = self.host {
			auth.push_str(&h.to_string());
		}

		if let Some(p) = self.port {
				auth.push(':');
				auth.push_str(&p.to_string());
		}

		write!(f, "{}", auth)
	}
}

impl ToSocketAddrs for Authority {
	type Iter = IntoIter<SocketAddr>;

	fn to_socket_addrs(&self) -> std::io::Result<IntoIter<SocketAddr>> {
		if let Some(ref host) = self.host {
			if let Some(port) = self.port {
				let socks = (host.as_str(), port as u16).to_socket_addrs()?;
				Ok(socks)
			} else {
				Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "URI did not specify a port number"))
			}
		} else {
			Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "URI did not specify a host name"))
		}
	}
}

fn parse_path(path: String) -> HierarchicalPart {
	println!("{}", path);

	if path.len() <= 1 {
		HierarchicalPart {
			authority: None,
			path_components: vec!(String::new())
		}
	} else {
		let path_bytes = path.as_bytes();
		let first_two = (path_bytes[0], path_bytes[1]);

		let mut authority = String::new();

		let mut slice_start: usize = 0;
		let p = &mut Vec::new();
		if first_two == (b'/', b'/') {
			slice_start = 2;
			for ix in 2..path_bytes.len() {
				let c = path_bytes[ix];
				match c {
					b'/' => {
						slice_start += 1;
						break;
					},
					_ => {
						p.push(c);
						slice_start += 1;
					}
				}
			}
			authority = String::from_utf8(p.clone()).unwrap();
		}

		let authority = parse_authority(authority);

		let mut path_components: Vec<String> = Vec::new();
		let path_bytes = &path_bytes[slice_start..];
		for path_comp in path_bytes.split(|b| b'/' == *b) {
			path_components.push(String::from_utf8(path_comp.to_vec()).unwrap());
		}

		HierarchicalPart {
			authority,
			path_components
		}

	}
}

fn parse_authority(authority: String) -> Option<Authority> {
	if authority.len() <= 0 {
		return None;
	}

	let auth_bytes = authority.as_bytes();
	
	let mut userinfo: Option<String> = None;
	let mut port: Option<usize> = None;

	let mut user_delim: Option<usize> = None;
	let mut port_delim: Option<usize> = None;
	for (i,c) in auth_bytes.into_iter().enumerate() {
		match *c {
			b'@' => {
				if let Some(_) = user_delim { panic!("Two user delims detected");}
				user_delim = Some(i);
			},
			b':' => port_delim = Some(i),
			_ => ()
		}
	}

	let mut host_start = 0;
	if let Some(u) = user_delim {
		if let Some(p) = port_delim {
			if u > p {port_delim = None;}
			userinfo = Some(String::from_utf8(auth_bytes[0..u].to_vec()).unwrap());
		}
		host_start = u + 1;
	}

	println!("{}\n", authority);
	//TODO: handle IPv6: [::zbc::]
	let mut host_end = auth_bytes.len();
	if let Some(p) = port_delim {
		host_end = p;
		if p+1 < auth_bytes.len() {
			let raw_port = auth_bytes[p+1..].to_vec();
			let raw_port = String::from_utf8(raw_port).unwrap();
			println!("{}\n", raw_port);
			let raw_port = usize::from_str_radix(&raw_port, 10).unwrap();
			port = Some(raw_port);
		}
	}

	let host_raw = (&auth_bytes[host_start..host_end]).to_vec();
	let mut host = None;
	if host_raw.len() > 0 {
		host = Some(String::from_utf8(host_raw).unwrap());	
	}
	

	Some(Authority {
		userinfo,
		host,
		port
	})
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn empty_path_only() {
    	let v = Uri::new("/");
        assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: None,
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::new())
        	}
        });
    }

	#[test]
	#[should_panic]
    fn invalid_url_start_delim() {
    	let _v = Uri::new(":");
    }    

    #[test]
	#[should_panic]
    fn invalid_url_start_digit() {
    	let _v = Uri::new("1");
    }

	#[test]
    fn path_url_scheme_delim_is_path_component() {
    	let v = Uri::new("/:c");
        assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: None,
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::from(":c"))
        	}
        });
    }    

    #[test]
    fn empty_url() {
    	let v = Uri::new("");
    	assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: None,
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::new())
        	}
        });
     }

     #[test]
     fn	multi_path() {
    	let v = Uri::new("/://c");
        assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: None,
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::from(":"),String::from(""),String::from("c"))
        	}
        });
    }
    #[test]
    fn recognize_scheme_start() {
    	let v = Uri::new("abc");
        assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::from(""))
        	}
        });
    }

    #[test]
    fn split_scheme_and_path() {
    	let v = Uri::new("abc:nbv/cfd");
        assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: None,
        		path_components: vec!(String::from("nbv"),String::from("cfd"))
        	}
        });
    }

    #[test]
    fn full_url_parsed_as_expected() {
    	let v = Uri::new("abc://user:test@test.com:8547/path/elem?query=test&query2=test2#fragment");
    	assert_eq!(v, Uri {
        	query: Some(String::from("query=test&query2=test2")),
        	fragment: Some(String::from("fragment")),
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: Some(Authority{
        			port: Some(8547_usize),
        			host: Some(String::from("test.com")),
        			userinfo: Some(String::from("user:test"))
        		}),
        		path_components: vec!(String::from("path"),String::from("elem"))
        	}
        });	
    }
    #[test]
    fn scheme_host_path_parsed_as_expected() {
    	let v = Uri::new("abc://test.com/path/elem");
    	assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: Some(Authority{
        			port: None,
        			host: Some(String::from("test.com")),
        			userinfo: None
        		}),
        		path_components: vec!(String::from("path"),String::from("elem"))
        	}
        });	
    }

    #[test]
    fn scheme_host_parsed_as_expected() {
    	let v = Uri::new("abc://test.com");
    	assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: Some(Authority{
        			port: None,
        			host: Some(String::from("test.com")),
        			userinfo: None
        		}),
        		path_components: vec!(String::new())
        	}
        });	
    }

    #[test]
    fn zero_len_port_parsed_as_expected() {
    	let v = Uri::new("abc://test.com:/path/elem");
    	assert_eq!(v, Uri {
        	query: None,
        	fragment: None,
        	scheme: Some(String::from("abc")),
        	path: HierarchicalPart {
        		authority: Some(Authority{
        			port: None,
        			host: Some(String::from("test.com")),
        			userinfo: None
        		}),
        		path_components: vec!(String::from("path"),String::from("elem"))
        	}
        });	
    }
}

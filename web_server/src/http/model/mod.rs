pub use self::request::*;
pub use self::response::*;

pub(crate) mod request;
pub(crate) mod response;



struct HttpHeader {
    
}

enum HttpHeaderType {
    Host,
    COnnection,
    Upgrade,
    Http2Settings,


}
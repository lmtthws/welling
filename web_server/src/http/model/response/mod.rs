pub use self::status_code::*;

pub(crate) mod status_code;


pub struct StatusLine {
    major_version: usize,
    minor_version: usize,
    status_code: StatusCode,
    reason: String
}

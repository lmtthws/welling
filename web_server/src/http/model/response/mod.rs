pub(crate) mod status_code;

pub use self::status_code::*;

use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct StatusLine {
    major_version: usize,
    minor_version: usize,
    status_code: StatusCode,
    reason: String
}

impl StatusLine {
    pub fn init(status_code: StatusCode, reason: String) -> StatusLine {
        StatusLine {
            major_version: 1,
            minor_version: 1,
            status_code: status_code,
            reason: reason
        }
    }
}

impl Display for StatusLine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "HTTP/{}.{} {} {}", self.major_version, self.minor_version, self.status_code.get_code(), self.reason)
    }
}

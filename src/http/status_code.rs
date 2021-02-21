use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not found",
        }
    }
}
// copy and clone --> copy value from stack and clone copy both heap and stack metadata, used for String type for instance
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

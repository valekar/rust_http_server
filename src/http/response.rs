use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // here dyn Write is a dynamic and impl Write is a
    //static dispatch , it could use traits of file or TCPStream or any thing that implements Write trait
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let mut body = "";

        if let Some(i) = &self.body {
            body = i;
        }

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n {}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

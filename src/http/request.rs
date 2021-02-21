//use str::{from_utf8, Utf8Error};

use super::method::{Method, MethodError};
use super::query_string::QueryString;
use std::error::Error;
use std::fmt::{Debug as FrmtDebug, Display, Formatter, Result as FrmtResult};
use std::str;
use std::str::Utf8Error;
use std::{convert::TryFrom, write};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        //let string = String::from("Hello");
        //string.encrypt();
        // -> 1
        /*  match str::from_utf8(buf) {
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        // -> 2
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {}
            Err(e) => return Err(e),
        }
        //-> 3
        // the question mark does the same as match here
        str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; */
        //-> 4
        let request = str::from_utf8(buf)?;

        // -> 1
        /*  match get_next_word(request) {
            Some((method, request)) => {}
            None => return Err(ParseError::InvalidRequest),
        } */

        // -> 2
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidRequest);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // -> 1
        /*  match path.find("?") {
            Some(i) => {
                query_string = Some(&path[i + 1..]);
                path = &path[..i]
            }
            None => {}
        }

        // we had write this because of None is matched, then it will be matched, it will panic and exit
        let q = path.find('?');
        if q.is_some() {
            let i = q.unwrap();
            query_string = Some(&path[i + 1..]);
            path = &path[..i]
        } */

        // -> 2
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i]
        }

        Ok(Self {
            path: path,
            query_string,
            method,
        })

        //unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // iterator
    /* let mut iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break,
        }
    } */

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..])); // +1 -> keep in mind that you incrementing 1 byte
        }
    }

    None

    //unimplemented!();
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FrmtResult {
        write!(f, "{}", self.message())
    }
}

impl FrmtDebug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FrmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

/* Trait example*/
// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!();
//     }
// }

// // impl Encrypt for &std {
// //     fn encrypt(&self) -> Self {
// //         unimplemented!();
// //     }
// // }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!();
//     }
// }

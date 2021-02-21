//use super::http::request::Request;
use crate::http::Request;
use std::convert::TryInto;
use std::net::TcpListener;
use std::{convert::TryFrom, io::Read};
//struct is like class
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    // two types functionality

    // associated functions - same as static types
    // this is a type assocaiated function because we are calling this using instance
    // Self -> alias for Struct name
    pub fn new(addr: String) -> Self {
        Self { addr: addr } // same as Server {addr}
    }

    // methods
    pub fn run(self) {
        println!("Server running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // we need to put the bounds here as try_from type is generic one
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("failed to parse the request {}", e);
                                }
                            } // conversion way 1
                              //let result: &Result<Request, _> = &buffer[..].try_into(); - conversion way 2
                        }
                        Err(e) => {
                            println!("failed to read from the request {}", e);
                        }
                    }
                    println!("ok");
                }
                Err(e) => {
                    println!("err {}", e);
                }
            }

            // let result = listener.accept();
            // if result.is_err() {
            //     continue;
            // }

            // let (stream, addr) = result.unwrap();
        }
    }
}

//arrays
//let a = [1,2,3];

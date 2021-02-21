#![allow(dead_code)]

use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let get = Method::GET;
    let post = Method::POST;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// tow types of strings

//1. string slice
// let string = String::from("127.0.0.1:8080")
// let string_slice = &string[10..];

// use string slice -> if you just want to read the value of a string
// use String -> for read, write, delete, append functionalities
// let string_borrow : &str = &string; -> this is valid

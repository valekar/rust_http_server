#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;
mod http;
mod server;
mod website_handler;
fn main() {
    //let get = Method::GET;
    //let post = Method::POST;

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

// tow types of strings

//1. string slice
// let string = String::from("127.0.0.1:8080")
// let string_slice = &string[10..];

// use string slice -> if you just want to read the value of a string
// use String -> for read, write, delete, append functionalities
// let string_borrow : &str = &string; -> this is valid

use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;
pub struct WebsiteHandler {
    pub_path: String,
}

impl WebsiteHandler {
    pub fn pub_path(&self) -> &String {
        &self.pub_path
    }
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            pub_path: public_path,
        }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", &self.pub_path(), file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(self.pub_path()) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, Some("Hello".to_string())),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }

        //Response::new(StatusCode::OK, Some("<h1>It works</h1>".to_string()))
    }
}

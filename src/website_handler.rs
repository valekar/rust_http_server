use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, Some("<h1>Root</h1>".to_string())),
                "/hello" => Response::new(StatusCode::OK, Some("Hello".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }

        //Response::new(StatusCode::OK, Some("<h1>It works</h1>".to_string()))
    }
}

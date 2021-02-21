use super::server::Handler;
use crate::http::{Request, Response, StatusCode};
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
        Response::new(StatusCode::OK, Some("<h1>It works</h1>".to_string()))
    }
}

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use regex_automata::{Regex, SparseDFA};

use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::uri::path::Path;

type RequestHandler = Box<dyn FnMut(Request) -> Response>;

#[derive(Debug, Clone)]
pub struct HttpRoute {
    pub method: Method,
    pub path_regex: Regex<SparseDFA<Vec<u8>>>,
}

pub struct HttpRouter {
    routes: Vec<(HttpRoute, RequestHandler)>,
    not_found_handler: RequestHandler,
}

impl HttpRouter {
    pub fn new(not_found_handler: RequestHandler) -> Self {
        Self {
            routes: vec![],
            not_found_handler,
        }
    }

    pub fn add_route(&mut self, route: HttpRoute, handler: RequestHandler) {
        self.routes.push((route, handler));
    }

    pub fn handle_request(&mut self, request: Request) -> Response {
        let request_path = match &request.uri.path {
            Some(path) => path.clone(),
            None => Path {
                src: "/".to_string(),
            },
        };

        for (route, handler) in self.routes.iter_mut() {
            if route.method == request.method
                && route
                    .path_regex
                    .is_match(request_path.to_string().as_bytes())
            {
                return handler(request);
            }
        }

        (self.not_found_handler)(request)
    }
}
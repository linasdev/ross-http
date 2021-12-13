extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::uri::path::Path;

type RequestHandler<'a> = Box<dyn FnMut(Request, Vec<String>) -> Response + 'a>;

#[derive(Debug, Clone)]
pub struct HttpRoute {
    pub method: Method,
    pub path: Path,
}

pub struct HttpRouter<'a> {
    routes: Vec<(HttpRoute, RequestHandler<'a>)>,
    not_found_handler: RequestHandler<'a>,
}

impl<'a> HttpRouter<'a> {
    pub fn new(not_found_handler: RequestHandler<'a>) -> Self {
        Self {
            routes: vec![],
            not_found_handler,
        }
    }

    pub fn add_route(&mut self, route: HttpRoute, handler: RequestHandler<'a>) {
        self.routes.push((route, handler));
    }

    pub fn handle_request(&mut self, request: Request) -> Response {
        let request_path = match &request.uri.path {
            Some(path) => path.clone(),
            None => Path { segments: vec![] },
        };

        for (route, handler) in self.routes.iter_mut() {
            if route.method == request.method {
                if route.path.segments.len() != request_path.segments.len() {
                    continue;
                }

                let mut variables = vec![];
                let mut matched = true;

                for (i, route_segment) in route.path.segments.iter().enumerate() {
                    let request_segment = request_path.segments.iter().nth(i).unwrap();

                    if *route_segment == "{}".to_string() {
                        variables.push(request_segment.clone());
                    } else if *route_segment == *request_segment {
                        continue;
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    return handler(request, variables);
                }
            }
        }

        (self.not_found_handler)(request, vec![])
    }
}

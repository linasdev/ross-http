extern crate alloc;

use core::convert::{TryFrom, From, TryInto};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version;
use crate::headers::Headers;
use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: Headers,
    pub body: String,
}

impl TryFrom<&str> for Request {
    type Error = HttpError;

    fn try_from(mut src: &str) -> Result<Self, Self::Error> {
        let method = if let Some(index) = src.find(" ") {
            let method_split = src.split_at(index);

            src = &method_split.1[1..];

            method_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);            
        };

        let path_and_query = if let Some(index) = src.find(" ") {
            let path_and_query_split = src.split_at(index);

            src = &path_and_query_split.1[1..];

            path_and_query_split.0
        } else {
            return Err(HttpError::Exhausted);            
        };

        let version = if let Some(index) = src.find("\r\n") {
            let version_split = src.split_at(index);

            src = &version_split.1[2..];

            version_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);            
        };

        let headers: Headers = if let Some(index) = src.find("\r\n\r\n") {
            let header_split = src.split_at(index);

            src = &header_split.1[4..];

            header_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);
        };

        let content_length = if let Some(content_length) = headers.headers.get("Content-Length") {
            match content_length.parse::<u32>() {
                Ok(content_length) => content_length,
                Err(_) => return Err(HttpError::InvalidRequest),
            }
        } else {
            0
        };

        let body = src.to_string();

        if (body.len() as u32) < content_length {
            return Err(HttpError::Exhausted);
        } else if (body.len() as u32) > content_length {
            return Err(HttpError::InvalidRequest);
        }

        let uri = if let Some(host_header) = headers.headers.get("Host") {
            (host_header.clone() + path_and_query).as_str().try_into()?
        } else {
            return Err(HttpError::InvalidRequest);
        };

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}

impl From<Request> for Vec<u8> {
    fn from(mut request: Request) -> Self {
        let mut data = vec![];

        let mut host_and_port = request.uri.authority.host;

        if let Some(port) = request.uri.authority.port {
            host_and_port += ":";
            host_and_port += port.as_str();
        }

        request.headers.headers.insert("Host".to_string(), host_and_port);
        if request.body.len() > 0 {
            request.headers.headers.insert("Content-Length".to_string(), request.body.len().to_string());
        }

        data.append(&mut request.method.into());
        data.append(&mut b" ".to_vec());
        if let Some(path) = request.uri.path {
            data.append(&mut path.into());
        } else {
            data.append(&mut b"/".to_vec());
        }
        if let Some(query) = request.uri.query {
            data.append(&mut b"?".to_vec());
            data.append(&mut query.into());
        }
        data.append(&mut b" ".to_vec());
        data.append(&mut request.version.into());
        data.append(&mut b"\r\n".to_vec());
        data.append(&mut request.headers.into());
        data.append(&mut b"\r\n".to_vec());
        data.append(&mut b"\r\n".to_vec());
        data.append(&mut request.body.into());

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::collections::BTreeMap;

    use crate::uri::authority::Authority;
    use crate::uri::path::Path;

    #[test]
    fn from_str_full_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Content-Length".to_string(), "4".to_string());
        headers.insert("Host".to_string(), "example.com".to_string());

        let method = Method::Post;
        let uri = Uri {
            scheme: None,
            authority: Authority {
                username: None,
                password: None,
                host: "example.com".to_string(),
                port: None,
            },
            path: Some(Path {
                src: "/resource".to_string()
            }),
            query: None,
        };
        let version = Version::Http11;
        let headers = Headers {
            headers,
        };
        let body = "Body".to_string();
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nContent-Length: 4\r\nHost: example.com\r\n\r\nBody"), Ok(Request {
            method,
            uri,
            version,
            headers,
            body,
        }));
    }

    #[test]
    fn to_bytes_full_test() {
        let method = Method::Post;
        let uri = Uri {
            scheme: None,
            authority: Authority {
                username: None,
                password: None,
                host: "example.com".to_string(),
                port: None,
            },
            path: Some(Path {
                src: "/resource".to_string()
            }),
            query: None,
        };
        let version = Version::Http11;
        let headers = Headers {
            headers: BTreeMap::new(),
        };
        let body = "Body".to_string();
        assert_eq!(Vec::<u8>::from(Request {
            method,
            uri,
            version,
            headers,
            body,
        }), b"POST /resource HTTP/1.1\r\nContent-Length: 4\r\nHost: example.com\r\n\r\nBody".to_vec());
    }

    #[test]
    fn from_str_no_body_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Host".to_string(), "example.com".to_string());

        let method = Method::Post;
        let uri = Uri {
            scheme: None,
            authority: Authority {
                username: None,
                password: None,
                host: "example.com".to_string(),
                port: None,
            },
            path: Some(Path {
                src: "/resource".to_string()
            }),
            query: None,
        };
        let version = Version::Http11;
        let headers = Headers {
            headers,
        };
        let body = "".to_string();
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nHost: example.com\r\n\r\n"), Ok(Request {
            method,
            uri,
            version,
            headers,
            body,
        }));
    }

    #[test]
    fn to_bytes_no_body_test() {
        let method = Method::Post;
        let uri = Uri {
            scheme: None,
            authority: Authority {
                username: None,
                password: None,
                host: "example.com".to_string(),
                port: None,
            },
            path: Some(Path {
                src: "/resource".to_string()
            }),
            query: None,
        };
        let version = Version::Http11;
        let headers = Headers {
            headers: BTreeMap::new(),
        };
        let body = "".to_string();
        assert_eq!(Vec::<u8>::from(Request {
            method,
            uri,
            version,
            headers,
            body,
        }), b"POST /resource HTTP/1.1\r\nHost: example.com\r\n\r\n".to_vec());
    }

    #[test]
    fn from_str_invalid_request1_test() {
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nHost: example.com\r\nContent-Length: 4\r\n\r\nBody "), Err(HttpError::InvalidRequest));
    }

    #[test]
    fn from_str_invalid_request2_test() {
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nContent-Length: 4\r\n\r\nBody"), Err(HttpError::InvalidRequest));
    }

    #[test]
    fn from_str_exhausted1_test() {
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nHost: example.com\r\nContent-Length: 4\r\n\r\nBod"), Err(HttpError::Exhausted));
    }

    #[test]
    fn from_str_exhausted2_test() {
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nHost: example.com\r\nContent-Length: 4"), Err(HttpError::Exhausted));
    }

    #[test]
    fn from_str_exhausted3_test() {
        assert_eq!(Request::try_from("POST /resource HTTP/1.1\r\nHost: example.com"), Err(HttpError::Exhausted));
    }
}

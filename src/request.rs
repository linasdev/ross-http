extern crate alloc;

use core::convert::{TryFrom, From, TryInto};

use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version;
use crate::headers::Headers;
use crate::error::HttpError;

pub struct Request<'a> {
    pub method: Method,
    pub uri: Uri<'a>,
    pub version: Version,
    pub headers: Headers<'a>,
    pub body: &'a str,
}

impl<'a> TryFrom<&'a str> for Request<'a> {
    type Error = HttpError;

    fn try_from(mut src: &'a str) -> Result<Self, Self::Error> {
        let (method, uri, version) = if let Some(index) = src.find("\r\n") {
            let first_line_split = src.split_at(index);

            src = &first_line_split.1[2..];

            let mut method_uri_version_split = first_line_split.0.split(" ");

            if method_uri_version_split.clone().count() == 3 {
                (
                    method_uri_version_split.nth(0).unwrap().try_into()?,
                    method_uri_version_split.nth(0).unwrap().try_into()?,
                    method_uri_version_split.nth(0).unwrap().try_into()?,
                )
            } else {
                return Err(HttpError::InvalidRequest);
            }
        } else {
            return Err(HttpError::InvalidRequest);
        };

        let mut headers_split = src.split("\r\n\r\n");

        let headers = if headers_split.clone().count() == 2 {
            src = headers_split.clone().nth(1).unwrap();

            headers_split.nth(0).unwrap().try_into()?
        } else {
            return Err(HttpError::InvalidRequest);
        };

        let body = src;

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}

impl<'a> From<Request<'a>> for Vec<u8> {
    fn from(mut request: Request<'a>) -> Self {
        let mut data = vec![];

        request.headers.headers.insert("Host", request.uri.authority.host);

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

    use crate::

    #[test]
    fn from_str_full_test() {
        let method = Method::Post;
        let uri = Uri {
            scheme: Scheme::
        }
        assert_eq!(Request::try_from("POST https://example.com/resource HTTP"), Ok(Request {
            method,
            uri,
            version,
            headers,
            body,
        }));
    }

    // #[test]
    // fn to_bytes_full_test() {
    //     assert_eq!(Vec::<u8>::from(Scheme::Http), b"http".to_vec());
    // }
}

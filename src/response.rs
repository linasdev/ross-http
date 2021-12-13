extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::convert::{From, TryFrom, TryInto};

use crate::error::HttpError;
use crate::status::Status;
use crate::headers::Headers;
use crate::version::Version;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: Headers,
    pub body: String,
}

impl TryFrom<&str> for Response {
    type Error = HttpError;

    fn try_from(mut src: &str) -> Result<Self, Self::Error> {
        let version = if let Some(index) = src.find(" ") {
            let version_split = src.split_at(index);

            src = &version_split.1[1..];

            version_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);
        };

        let skip_headers = src.find("\r\n") == src.find("\r\n\r\n");
        
        let status = if let Some(index) = src.find("\r\n") {
            let status_split = src.split_at(index);

            src = &status_split.1[2..];

            status_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);
        };

        let headers: Headers = if skip_headers {
            src = &src[2..];
            Headers { headers: BTreeMap::new() }
        } else {
            if let Some(index) = src.find("\r\n\r\n") {
                let header_split = src.split_at(index);

                src = &header_split.1[4..];

                header_split.0.try_into()?
            } else {
                return Err(HttpError::Exhausted);
            }
        };

        let content_length = if let Some(content_length) = headers.headers.get("Content-Length") {
            match content_length.parse::<u32>() {
                Ok(content_length) => content_length,
                Err(_) => return Err(HttpError::InvalidResponse),
            }
        } else {
            0
        };

        let body = src.to_string();

        if (body.len() as u32) < content_length {
            return Err(HttpError::Exhausted);
        } else if (body.len() as u32) > content_length {
            return Err(HttpError::InvalidResponse);
        }

        Ok(Self {
            version,
            status,
            headers,
            body,
        })
    }
}

impl From<Response> for Vec<u8> {
    fn from(mut response: Response) -> Self {
        let mut data = vec![];

        if response.body.len() > 0 {
            response
                .headers
                .headers
                .insert("Content-Length".to_string(), response.body.len().to_string());
        }

        data.append(&mut response.version.into());
        data.append(&mut b" ".to_vec());
        data.append(&mut response.status.into());
        if response.headers.headers.len() > 0 {
            data.append(&mut b"\r\n".to_vec());
            data.append(&mut response.headers.into());
        }
        data.append(&mut b"\r\n".to_vec());
        data.append(&mut b"\r\n".to_vec());
        data.append(&mut response.body.into());

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::collections::BTreeMap;

    use crate::status::StatusCode;

    #[test]
    fn from_str_full_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Content-Length".to_string(), "4".to_string());

        let version = Version::Http11;
        let status = Status::from(StatusCode::Ok);
        let headers = Headers { headers };
        let body = "Body".to_string();
        assert_eq!(
            Response::try_from(
                "HTTP/1.1 200 Ok\r\nContent-Length: 4\r\n\r\nBody"
            ),
            Ok(Response {
                version,
                status,
                headers,
                body,
            }),
        );
    }

    #[test]
    fn to_bytes_full_test() {
        let version = Version::Http11;
        let status = Status::from(StatusCode::Ok);
        let headers = Headers { headers: BTreeMap::new() };
        let body = "Body".to_string();
        assert_eq!(
            Vec::<u8>::from(Response {
                    version,
                    status,
                    headers,
                    body,
            }),
            b"HTTP/1.1 200 Ok\r\nContent-Length: 4\r\n\r\nBody".to_vec()
        );
    }

    #[test]
    fn from_str_no_body_test() {
        let version = Version::Http11;
        let status = Status::from(StatusCode::Ok);
        let headers = Headers { headers: BTreeMap::new() };
        let body = "".to_string();
        assert_eq!(
            Response::try_from(
                "HTTP/1.1 200 Ok\r\n\r\n"
            ),
            Ok(Response {
                version,
                status,
                headers,
                body,
            }),
        );
    }

    #[test]
    fn to_bytes_no_body_test() {
        let version = Version::Http11;
        let status = Status::from(StatusCode::Ok);
        let headers = Headers { headers: BTreeMap::new() };
        let body = "".to_string();
        assert_eq!(
            Vec::<u8>::from(Response {
                    version,
                    status,
                    headers,
                    body,
            }),
            b"HTTP/1.1 200 Ok\r\n\r\n".to_vec(),
        );
    }

    #[test]
    fn from_str_invalid_request1_test() {
        assert_eq!(
            Response::try_from(
                "HTTP/1.1 200 Ok\r\nContent-Length: 4\r\n\r\nBody "
            ),
            Err(HttpError::InvalidResponse)
        );
    }

    #[test]
    fn from_str_exhausted1_test() {
        assert_eq!(
            Response::try_from(
                "HTTP/1.1 200 Ok\r\nContent-Length: 4\r\n\r\nBod"
            ),
            Err(HttpError::Exhausted)
        );
    }

    #[test]
    fn from_str_exhausted2_test() {
        assert_eq!(
            Response::try_from("HTTP/1.1 200 Ok\r\nContent-Length: 4"),
            Err(HttpError::Exhausted)
        );
    }

    #[test]
    fn from_str_exhausted3_test() {
        assert_eq!(
            Response::try_from("HTTP/1.1 200 Ok\r\n"),
            Err(HttpError::Exhausted)
        );
    }
}

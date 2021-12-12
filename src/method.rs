extern crate alloc;

use alloc::vec::Vec;
use core::convert::{From, TryFrom};

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}

impl TryFrom<&str> for Method {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            "CONNECT" => Ok(Method::Connect),
            "PATCH" => Ok(Method::Patch),
            "TRACE" => Ok(Method::Trace),
            _ => Err(HttpError::InvalidMethod),
        }
    }
}

impl From<Method> for Vec<u8> {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => b"GET".to_vec(),
            Method::Post => b"POST".to_vec(),
            Method::Put => b"PUT".to_vec(),
            Method::Delete => b"DELETE".to_vec(),
            Method::Head => b"HEAD".to_vec(),
            Method::Options => b"OPTIONS".to_vec(),
            Method::Connect => b"CONNECT".to_vec(),
            Method::Patch => b"PATCH".to_vec(),
            Method::Trace => b"TRACE".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_get_test() {
        assert_eq!(Method::try_from("GET"), Ok(Method::Get));
    }

    #[test]
    fn to_bytes_get_test() {
        assert_eq!(Vec::<u8>::from(Method::Get), b"GET".to_vec());
    }

    #[test]
    fn from_str_post_test() {
        assert_eq!(Method::try_from("POST"), Ok(Method::Post));
    }

    #[test]
    fn to_bytes_post_test() {
        assert_eq!(Vec::<u8>::from(Method::Post), b"POST".to_vec());
    }

    #[test]
    fn from_str_put_test() {
        assert_eq!(Method::try_from("PUT"), Ok(Method::Put));
    }

    #[test]
    fn to_bytes_put_test() {
        assert_eq!(Vec::<u8>::from(Method::Put), b"PUT".to_vec());
    }

    #[test]
    fn from_str_delete_test() {
        assert_eq!(Method::try_from("DELETE"), Ok(Method::Delete));
    }

    #[test]
    fn to_bytes_delete_test() {
        assert_eq!(Vec::<u8>::from(Method::Delete), b"DELETE".to_vec());
    }

    #[test]
    fn from_str_head_test() {
        assert_eq!(Method::try_from("HEAD"), Ok(Method::Head));
    }

    #[test]
    fn to_bytes_head_test() {
        assert_eq!(Vec::<u8>::from(Method::Head), b"HEAD".to_vec());
    }

    #[test]
    fn from_str_options_test() {
        assert_eq!(Method::try_from("OPTIONS"), Ok(Method::Options));
    }

    #[test]
    fn to_bytes_options_test() {
        assert_eq!(Vec::<u8>::from(Method::Options), b"OPTIONS".to_vec());
    }

    #[test]
    fn from_str_connect_test() {
        assert_eq!(Method::try_from("CONNECT"), Ok(Method::Connect));
    }

    #[test]
    fn to_bytes_connect_test() {
        assert_eq!(Vec::<u8>::from(Method::Connect), b"CONNECT".to_vec());
    }

    #[test]
    fn from_str_patch_test() {
        assert_eq!(Method::try_from("PATCH"), Ok(Method::Patch));
    }

    #[test]
    fn to_bytes_patch_test() {
        assert_eq!(Vec::<u8>::from(Method::Patch), b"PATCH".to_vec());
    }

    #[test]
    fn from_str_trace_test() {
        assert_eq!(Method::try_from("TRACE"), Ok(Method::Trace));
    }

    #[test]
    fn to_bytes_trace_test() {
        assert_eq!(Vec::<u8>::from(Method::Trace), b"TRACE".to_vec());
    }

    #[test]
    fn from_str_invalid_method_test() {
        assert_eq!(Method::try_from("METHOD"), Err(HttpError::InvalidMethod));
    }
}

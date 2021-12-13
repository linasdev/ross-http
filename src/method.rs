extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Delete => "DELETE".to_string(),
            Method::Head => "HEAD".to_string(),
            Method::Options => "OPTIONS".to_string(),
            Method::Connect => "CONNECT".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Trace => "TRACE".to_string(),
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
    fn to_string_get_test() {
        assert_eq!(Method::Get.to_string(), "GET".to_string());
    }

    #[test]
    fn from_str_post_test() {
        assert_eq!(Method::try_from("POST"), Ok(Method::Post));
    }

    #[test]
    fn to_string_post_test() {
        assert_eq!(Method::Post.to_string(), "POST".to_string());
    }

    #[test]
    fn from_str_put_test() {
        assert_eq!(Method::try_from("PUT"), Ok(Method::Put));
    }

    #[test]
    fn to_string_put_test() {
        assert_eq!(Method::Put.to_string(), "PUT".to_string());
    }

    #[test]
    fn from_str_delete_test() {
        assert_eq!(Method::try_from("DELETE"), Ok(Method::Delete));
    }

    #[test]
    fn to_string_delete_test() {
        assert_eq!(Method::Delete.to_string(), "DELETE".to_string());
    }

    #[test]
    fn from_str_head_test() {
        assert_eq!(Method::try_from("HEAD"), Ok(Method::Head));
    }

    #[test]
    fn to_string_head_test() {
        assert_eq!(Method::Head.to_string(), "HEAD".to_string());
    }

    #[test]
    fn from_str_options_test() {
        assert_eq!(Method::try_from("OPTIONS"), Ok(Method::Options));
    }

    #[test]
    fn to_string_options_test() {
        assert_eq!(Method::Options.to_string(), "OPTIONS".to_string());
    }

    #[test]
    fn from_str_connect_test() {
        assert_eq!(Method::try_from("CONNECT"), Ok(Method::Connect));
    }

    #[test]
    fn to_string_connect_test() {
        assert_eq!(Method::Connect.to_string(), "CONNECT".to_string());
    }

    #[test]
    fn from_str_patch_test() {
        assert_eq!(Method::try_from("PATCH"), Ok(Method::Patch));
    }

    #[test]
    fn to_string_patch_test() {
        assert_eq!(Method::Patch.to_string(), "PATCH".to_string());
    }

    #[test]
    fn from_str_trace_test() {
        assert_eq!(Method::try_from("TRACE"), Ok(Method::Trace));
    }

    #[test]
    fn to_string_trace_test() {
        assert_eq!(Method::Trace.to_string(), "TRACE".to_string());
    }

    #[test]
    fn from_str_invalid_method_test() {
        assert_eq!(Method::try_from("METHOD"), Err(HttpError::InvalidMethod));
    }
}

extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
pub enum Version {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30,
}

impl TryFrom<&str> for Version {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "HTTP/0.9" => Ok(Version::Http09),
            "HTTP/1.0" => Ok(Version::Http10),
            "HTTP/1.1" => Ok(Version::Http11),
            "HTTP/2.0" => Ok(Version::Http20),
            "HTTP/3.0" => Ok(Version::Http30),
            _ => Err(HttpError::InvalidVersion),
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::Http09 => "HTTP/0.9".to_string(),
            Version::Http10 => "HTTP/1.0".to_string(),
            Version::Http11 => "HTTP/1.1".to_string(),
            Version::Http20 => "HTTP/2.0".to_string(),
            Version::Http30 => "HTTP/3.0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_http09_test() {
        assert_eq!(Version::try_from("HTTP/0.9"), Ok(Version::Http09));
    }

    #[test]
    fn to_string_http09_test() {
        assert_eq!(Version::Http09.to_string(), "HTTP/0.9".to_string());
    }

    #[test]
    fn from_str_http10_test() {
        assert_eq!(Version::try_from("HTTP/1.0"), Ok(Version::Http10));
    }

    #[test]
    fn to_string_http10_test() {
        assert_eq!(Version::Http10.to_string(), "HTTP/1.0".to_string());
    }

    #[test]
    fn from_str_http11_test() {
        assert_eq!(Version::try_from("HTTP/1.1"), Ok(Version::Http11));
    }

    #[test]
    fn to_string_http11_test() {
        assert_eq!(Version::Http11.to_string(), "HTTP/1.1".to_string());
    }

    #[test]
    fn from_str_http20_test() {
        assert_eq!(Version::try_from("HTTP/2.0"), Ok(Version::Http20));
    }

    #[test]
    fn to_string_http20_test() {
        assert_eq!(Version::Http20.to_string(), "HTTP/2.0".to_string());
    }

    #[test]
    fn from_str_http30_test() {
        assert_eq!(Version::try_from("HTTP/3.0"), Ok(Version::Http30));
    }

    #[test]
    fn to_string_http30_test() {
        assert_eq!(Version::Http30.to_string(), "HTTP/3.0".to_string());
    }

    #[test]
    fn from_str_invalid_version_test() {
        assert_eq!(
            Version::try_from("HTTP/3.1"),
            Err(HttpError::InvalidVersion)
        );
    }
}

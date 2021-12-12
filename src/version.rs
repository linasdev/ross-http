extern crate alloc;

use alloc::vec::Vec;
use core::convert::{From, TryFrom};

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
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

impl From<Version> for Vec<u8> {
    fn from(version: Version) -> Self {
        match version {
            Version::Http09 => b"HTTP/0.9".to_vec(),
            Version::Http10 => b"HTTP/1.0".to_vec(),
            Version::Http11 => b"HTTP/1.1".to_vec(),
            Version::Http20 => b"HTTP/2.0".to_vec(),
            Version::Http30 => b"HTTP/3.0".to_vec(),
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
    fn to_bytes_http09_test() {
        assert_eq!(Vec::<u8>::from(Version::Http09), b"HTTP/0.9".to_vec());
    }

    #[test]
    fn from_str_http10_test() {
        assert_eq!(Version::try_from("HTTP/1.0"), Ok(Version::Http10));
    }

    #[test]
    fn to_bytes_http10_test() {
        assert_eq!(Vec::<u8>::from(Version::Http10), b"HTTP/1.0".to_vec());
    }

    #[test]
    fn from_str_http11_test() {
        assert_eq!(Version::try_from("HTTP/1.1"), Ok(Version::Http11));
    }

    #[test]
    fn to_bytes_http11_test() {
        assert_eq!(Vec::<u8>::from(Version::Http11), b"HTTP/1.1".to_vec());
    }

    #[test]
    fn from_str_http20_test() {
        assert_eq!(Version::try_from("HTTP/2.0"), Ok(Version::Http20));
    }

    #[test]
    fn to_bytes_http20_test() {
        assert_eq!(Vec::<u8>::from(Version::Http20), b"HTTP/2.0".to_vec());
    }

    #[test]
    fn from_str_http30_test() {
        assert_eq!(Version::try_from("HTTP/3.0"), Ok(Version::Http30));
    }

    #[test]
    fn to_bytes_http30_test() {
        assert_eq!(Vec::<u8>::from(Version::Http30), b"HTTP/3.0".to_vec());
    }

    #[test]
    fn from_str_invalid_version_test() {
        assert_eq!(
            Version::try_from("HTTP/3.1"),
            Err(HttpError::InvalidVersion)
        );
    }
}

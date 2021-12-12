extern crate alloc;

use alloc::vec::Vec;
use core::convert::{TryFrom, From};

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub enum Scheme {
    Http,
    Https,
}

impl TryFrom<&str> for Scheme {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "http" => Ok(Scheme::Http),
            "https" => Ok(Scheme::Https),
            _ => Err(HttpError::InvalidScheme),
        }
    }
}

impl From<Scheme> for Vec<u8> {
    fn from(scheme: Scheme) -> Self {
        match scheme {
            Scheme::Http => b"http".to_vec(),
            Scheme::Https => b"https".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_http_test() {
        assert_eq!(Scheme::try_from("http"), Ok(Scheme::Http));
    }

    #[test]
    fn to_bytes_http_test() {
        assert_eq!(Vec::<u8>::from(Scheme::Http), b"http".to_vec());
    }

    #[test]
    fn from_str_https_test() {
        assert_eq!(Scheme::try_from("https"), Ok(Scheme::Https));
    }

    #[test]
    fn to_bytes_https_test() {
        assert_eq!(Vec::<u8>::from(Scheme::Https), b"https".to_vec());
    }

    #[test]
    fn from_str_invalid_scheme_test() {
        assert_eq!(Scheme::try_from("smtp"), Err(HttpError::InvalidScheme));
    }
}

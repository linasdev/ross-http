extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
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

impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Scheme::Http => "http".to_string(),
            Scheme::Https => "https".to_string(),
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
    fn to_string_http_test() {
        assert_eq!(Scheme::Http.to_string(), "http".to_string());
    }

    #[test]
    fn from_str_https_test() {
        assert_eq!(Scheme::try_from("https"), Ok(Scheme::Https));
    }

    #[test]
    fn to_string_https_test() {
        assert_eq!(Scheme::Https.to_string(), "https".to_string());
    }

    #[test]
    fn from_str_invalid_scheme_test() {
        assert_eq!(Scheme::try_from("smtp"), Err(HttpError::InvalidScheme));
    }
}

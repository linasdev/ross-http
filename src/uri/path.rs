extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub src: String,
}

impl TryFrom<&str> for Path {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            src: src.to_string(),
        })
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.src.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(
            Path::try_from("/resource/subresource"),
            Ok(Path {
                src: "/resource/subresource".to_string()
            })
        );
    }

    #[test]
    fn to_string_test() {
        assert_eq!(
            Path {
                src: "/resource/subresource".to_string()
            }.to_string(),
            "/resource/subresource".to_string()
        );
    }
}

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<String>,
}

impl TryFrom<&str> for Path {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut iterator = src.split("/").map(|segment| segment.to_string());
        iterator.next();
        let segments = iterator.collect();
    
        Ok(Self {
            segments,
        })
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        let mut data = String::new();

        data += "/";

        let mut iterator = self.segments.iter();

        if let Some(segment) = iterator.next() {
            data += segment.as_str();
        }

        for segment in iterator {
            data += "/";
            data += segment.as_str();
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    #[test]
    fn from_str_test() {
        assert_eq!(
            Path::try_from("/resource/subresource"),
            Ok(Path {
                segments: vec!["resource".to_string(), "subresource".to_string()],
            })
        );
    }

    #[test]
    fn to_string_test() {
        assert_eq!(
            Path {
                segments: vec!["resource".to_string(), "subresource".to_string()],
            }
            .to_string(),
            "/resource/subresource".to_string()
        );
    }

    #[test]
    fn from_str_empty_test() {
        assert_eq!(
            Path::try_from(""),
            Ok(Path {
                segments: vec![],
            })
        );
    }

    #[test]
    fn from_str_root_test() {
        assert_eq!(
            Path::try_from("/"),
            Ok(Path {
                segments: vec![],
            })
        );
    }

    #[test]
    fn to_string_empty_test() {
        assert_eq!(
            Path {
                segments: vec![],
            }
            .to_string(),
            "/".to_string()
        );
    }
}

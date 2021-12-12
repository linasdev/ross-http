extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Path {
    pub src: String,
}

impl TryFrom<&str> for Path {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Self { src: src.to_string() })
    }
}

impl<'a> From<Path> for Vec<u8> {
    fn from(path: Path) -> Self {
        path.src.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(Path::try_from("/resource/subresource"), Ok(Path { src: "/resource/subresource".to_string() }));
    }

    #[test]
    fn to_bytes_test() {
        assert_eq!(Vec::<u8>::from(Path { src: "/resource/subresource".to_string() }), b"/resource/subresource".to_vec());
    }
}

extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Path<'a> {
    pub src: &'a str,
}

impl<'a> TryFrom<&'a str> for Path<'a> {
    type Error = HttpError;

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { src })
    }
}

impl<'a> From<Path<'a>> for Vec<u8> {
    fn from(path: Path<'a>) -> Self {
        path.src.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(Path::try_from("/resource/subresource"), Ok(Path { src: "/resource/subresource" }));
    }

    #[test]
    fn to_bytes_test() {
        assert_eq!(Vec::<u8>::from(Path { src: "/resource/subresource" }), b"/resource/subresource".to_vec());
    }
}

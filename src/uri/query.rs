extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Query<'a> {
    pub src: &'a str,
}

impl<'a> TryFrom<&'a str> for Query<'a> {
    type Error = HttpError;

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { src })
    }
}

impl<'a> From<Query<'a>> for Vec<u8> {
    fn from(path: Query<'a>) -> Self {
        path.src.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(Query::try_from("parameter1=value1&parameter2=value2"), Ok(Query { src: "parameter1=value1&parameter2=value2" }));
    }

    #[test]
    fn to_bytes_test() {
        assert_eq!(Vec::<u8>::from(Query { src: "parameter1=value1&parameter2=value2" }), b"parameter1=value1&parameter2=value2".to_vec());
    }
}

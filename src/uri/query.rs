extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Query<'a> {
    pub parameters: BTreeMap<&'a str, &'a str>,
}

impl<'a> TryFrom<&'a str> for Query<'a> {
    type Error = HttpError;

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        let mut parameters = BTreeMap::new();

        for parameter_str in src.split("&") {
            let parameter_parts: Vec<&str> = parameter_str.split("=").collect();

            if parameter_parts.len() != 2 {
                return Err(HttpError::InvalidQuery);
            }

            parameters.insert(parameter_parts[0], parameter_parts[1]);
        }

        Ok(Self { parameters })
    }
}

impl<'a> From<Query<'a>> for Vec<u8> {
    fn from(query: Query<'a>) -> Self {
        let mut data = vec![];

        let mut iterator = query.parameters.iter();

        if let Some((parameter, value)) = iterator.next() {
            data.append(&mut parameter.as_bytes().to_vec());
            data.append(&mut b"=".to_vec());
            data.append(&mut value.as_bytes().to_vec());
        }

        for (parameter, value) in iterator {
            data.append(&mut b"&".to_vec());
            data.append(&mut parameter.as_bytes().to_vec());
            data.append(&mut b"=".to_vec());
            data.append(&mut value.as_bytes().to_vec());
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_single_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter", "value");
        assert_eq!(Query::try_from("parameter=value"), Ok(Query { parameters }));
    }

    #[test]
    fn to_bytes_single_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter", "value");
        assert_eq!(Vec::<u8>::from(Query { parameters }), b"parameter=value".to_vec());
    }

    #[test]
    fn from_str_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");
        assert_eq!(Query::try_from("parameter1=value1&parameter2=value2"), Ok(Query { parameters }));
    }

    #[test]
    fn to_bytes_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");
        assert_eq!(Vec::<u8>::from(Query { parameters }), b"parameter1=value1&parameter2=value2".to_vec());
    }
}

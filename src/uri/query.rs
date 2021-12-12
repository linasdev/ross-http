extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub parameters: BTreeMap<String, String>,
}

impl TryFrom<&str> for Query {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut parameters = BTreeMap::new();

        for parameter_str in src.split("&") {
            let parameter_parts: Vec<&str> = parameter_str.split("=").collect();

            if parameter_parts.len() != 2 {
                return Err(HttpError::InvalidQuery);
            }

            parameters.insert(parameter_parts[0].to_string(), parameter_parts[1].to_string());
        }

        Ok(Self { parameters })
    }
}

impl From<Query> for Vec<u8> {
    fn from(query: Query) -> Self {
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
        parameters.insert("parameter".to_string(), "value".to_string());
        assert_eq!(Query::try_from("parameter=value"), Ok(Query { parameters }));
    }

    #[test]
    fn to_bytes_single_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter".to_string(), "value".to_string());
        assert_eq!(Vec::<u8>::from(Query { parameters }), b"parameter=value".to_vec());
    }

    #[test]
    fn from_str_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1".to_string(), "value1".to_string());
        parameters.insert("parameter2".to_string(), "value2".to_string());
        assert_eq!(Query::try_from("parameter1=value1&parameter2=value2"), Ok(Query { parameters }));
    }

    #[test]
    fn to_bytes_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1".to_string(), "value1".to_string());
        parameters.insert("parameter2".to_string(), "value2".to_string());
        assert_eq!(Vec::<u8>::from(Query { parameters }), b"parameter1=value1&parameter2=value2".to_vec());
    }
}

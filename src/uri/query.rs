extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
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

            parameters.insert(
                parameter_parts[0].to_string(),
                parameter_parts[1].to_string(),
            );
        }

        Ok(Self { parameters })
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut data = String::new();

        let mut iterator = self.parameters.iter();

        if let Some((parameter, value)) = iterator.next() {
            data += parameter;
            data += "=";
            data += value;
        }

        for (parameter, value) in iterator {
            data += "&";
            data += parameter;
            data += "=";
            data += value;
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
    fn to_string_single_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter".to_string(), "value".to_string());
        assert_eq!(
            Query { parameters }.to_string(),
            "parameter=value".to_string()
        );
    }

    #[test]
    fn from_str_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1".to_string(), "value1".to_string());
        parameters.insert("parameter2".to_string(), "value2".to_string());
        assert_eq!(
            Query::try_from("parameter1=value1&parameter2=value2"),
            Ok(Query { parameters })
        );
    }

    #[test]
    fn to_string_multiple_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1".to_string(), "value1".to_string());
        parameters.insert("parameter2".to_string(), "value2".to_string());
        assert_eq!(
            Query { parameters }.to_string(),
            "parameter1=value1&parameter2=value2".to_string()
        );
    }
}

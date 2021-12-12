extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Headers {
    pub headers: BTreeMap<String, String>,
}

impl TryFrom<&str> for Headers {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut headers = BTreeMap::new();

        for header_line in src.split("\r\n") {
            let header_parts: Vec<&str> = header_line.split(": ").collect();

            if header_parts.len() != 2 {
                return Err(HttpError::InvalidHeader);
            }

            headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
        }

        Ok(Self { headers })
    }
}

impl From<Headers> for Vec<u8> {
    fn from(headers: Headers) -> Self {
        let mut data = vec![];

        let mut iterator = headers.headers.iter();

        if let Some((key, value)) = iterator.next() {
            data.append(&mut key.as_bytes().to_vec());
            data.append(&mut b": ".to_vec());
            data.append(&mut value.as_bytes().to_vec());
        }

        for (key, value) in iterator {
            data.append(&mut b"\r\n".to_vec());
            data.append(&mut key.as_bytes().to_vec());
            data.append(&mut b": ".to_vec());
            data.append(&mut value.as_bytes().to_vec());
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_headers_single_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name".to_string(), "Header-Value".to_string());
        assert_eq!(Headers::try_from("Header-Name: Header-Value"), Ok(Headers { headers }));
    }

    #[test]
    fn to_bytes_headers_single_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name".to_string(), "Header-Value".to_string());
        assert_eq!(Vec::<u8>::from(Headers { headers }), b"Header-Name: Header-Value".to_vec());
    }

    #[test]
    fn from_str_headers_multiple_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name1".to_string(), "Header-Value1".to_string());
        headers.insert("Header-Name2".to_string(), "Header-Value2".to_string());
        assert_eq!(Headers::try_from("Header-Name1: Header-Value1\r\nHeader-Name2: Header-Value2"), Ok(Headers { headers }));
    }

    #[test]
    fn to_bytes_headers_multiple_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name1".to_string(), "Header-Value1".to_string());
        headers.insert("Header-Name2".to_string(), "Header-Value2".to_string());
        assert_eq!(Vec::<u8>::from(Headers { headers }), b"Header-Name1: Header-Value1\r\nHeader-Name2: Header-Value2".to_vec());
    }

    #[test]
    fn from_str_invalid_header_test() {
        assert_eq!(Headers::try_from("Header-Name Header-Value"), Err(HttpError::InvalidHeader));
    }
}

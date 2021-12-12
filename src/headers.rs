extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::collections::BTreeMap;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Headers<'a> {
    pub headers: BTreeMap<&'a str, &'a str>,
}

impl<'a> TryFrom<&'a str> for Headers<'a> {
    type Error = HttpError;

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        let mut headers = BTreeMap::new();

        for header_line in src.split("\r\n") {
            let header_parts: Vec<&str> = header_line.split(": ").collect();

            if header_parts.len() != 2 {
                return Err(HttpError::InvalidHeader);
            }

            headers.insert(header_parts[0], header_parts[1]);
        }

        Ok(Self { headers })
    }
}

impl<'a> From<Headers<'a>> for Vec<u8> {
    fn from(headers: Headers<'a>) -> Self {
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
        headers.insert("Header-Name", "Header-Value");
        assert_eq!(Headers::try_from("Header-Name: Header-Value"), Ok(Headers { headers }));
    }

    #[test]
    fn to_bytes_headers_single_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name", "Header-Value");
        assert_eq!(Vec::<u8>::from(Headers { headers }), b"Header-Name: Header-Value".to_vec());
    }

    #[test]
    fn from_str_headers_multiple_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name1", "Header-Value1");
        headers.insert("Header-Name2", "Header-Value2");
        assert_eq!(Headers::try_from("Header-Name1: Header-Value1\r\nHeader-Name2: Header-Value2"), Ok(Headers { headers }));
    }

    #[test]
    fn to_bytes_headers_multiple_test() {
        let mut headers = BTreeMap::new();
        headers.insert("Header-Name1", "Header-Value1");
        headers.insert("Header-Name2", "Header-Value2");
        assert_eq!(Vec::<u8>::from(Headers { headers }), b"Header-Name1: Header-Value1\r\nHeader-Name2: Header-Value2".to_vec());
    }

    #[test]
    fn from_str_invalid_header_test() {
        assert_eq!(Headers::try_from("Header-Name Header-Value"), Err(HttpError::InvalidHeader));
    }
}

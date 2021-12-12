extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::uri::scheme::Scheme;
use crate::uri::authority::Authority;
use crate::uri::path::Path;
use crate::uri::query::Query;
use crate::error::HttpError;

pub mod scheme;
pub mod authority;
pub mod path;
pub mod query;

#[derive(Debug, PartialEq)]
pub struct Uri<'a> {
    pub scheme: Scheme,
    pub authority: Authority<'a>,
    pub path: Option<Path<'a>>,
    pub query: Option<Query<'a>>,
}

impl<'a> TryFrom<&'a str> for Uri<'a> {
    type Error = HttpError;

    fn try_from(mut src: &'a str) -> Result<Self, Self::Error> {
        let mut scheme_split = src.split("://");

        let scheme = match scheme_split.nth(0) {
            Some(src) => src.try_into()?,
            None => return Err(HttpError::InvalidUri),
        };

        src = match scheme_split.nth(0) {
            Some(src) => src,
            None => return Err(HttpError::InvalidUri),
        };

        let mut query_split = src.split("?");

        let query = if query_split.clone().count() == 2 {
            src = query_split.nth(0).unwrap();
            Some(query_split.nth(0).unwrap().try_into()?)
        } else if query_split.count() == 1 {
            None
        } else {
            return Err(HttpError::InvalidUri);
        };

        let authority = if let Some(index) = src.find("/") {
            let authority_split = src.split_at(index);

            src = authority_split.1;
            authority_split.0.try_into()?
        } else {
            let authority = src.try_into()?;
            src = "";
            authority
        };

        let path = if src.len() > 0 {
            Some(src.try_into()?)
        } else {
            None
        };

        Ok(Self {
            scheme,
            authority,
            path,
            query,
        })
    }
}

impl<'a> From<Uri<'a>> for Vec<u8> {
    fn from(uri: Uri) -> Self {
        let mut data = vec![];

        data.append(&mut uri.scheme.into());
        data.append(&mut b"://".to_vec());
        data.append(&mut uri.authority.into());
        if let Some(path) = uri.path {
            data.append(&mut path.into());
        }
        if let Some(query) = uri.query {
            data.append(&mut b"?".to_vec());
            data.append(&mut query.into());
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::collections::BTreeMap;

    #[test]
    fn from_str_full_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");

        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = Some(Path { src: "/resource/subresource" });
        let query = Some(Query { parameters });
        assert_eq!(Uri::try_from("https://username:password@example.com:123/resource/subresource?parameter1=value1&parameter2=value2"), Ok(Uri {
            scheme,
            authority,
            path,
            query,
        }));
    }

    #[test]
    fn to_bytes_full_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");

        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = Some(Path { src: "/resource/subresource" });
        let query = Some(Query { parameters });
        assert_eq!(Vec::<u8>::from(Uri {
            scheme,
            authority,
            path,
            query,
        }), b"https://username:password@example.com:123/resource/subresource?parameter1=value1&parameter2=value2".to_vec());
    }

    #[test]
    fn from_str_no_path_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");

        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = None;
        let query = Some(Query { parameters });
        assert_eq!(Uri::try_from("https://username:password@example.com:123?parameter1=value1&parameter2=value2"), Ok(Uri {
            scheme,
            authority,
            path,
            query,
        }));
    }

    #[test]
    fn to_bytes_no_path_test() {
        let mut parameters = BTreeMap::new();
        parameters.insert("parameter1", "value1");
        parameters.insert("parameter2", "value2");

        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = None;
        let query = Some(Query { parameters });
        assert_eq!(Vec::<u8>::from(Uri {
            scheme,
            authority,
            path,
            query,
        }), b"https://username:password@example.com:123?parameter1=value1&parameter2=value2".to_vec());
    }

    #[test]
    fn from_str_no_query_test() {
        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = None;
        let query = None;
        assert_eq!(Uri::try_from("https://username:password@example.com:123"), Ok(Uri {
            scheme,
            authority,
            path,
            query,
        }));
    }

    #[test]
    fn to_bytes_no_query_test() {
        let scheme = Scheme::Https;
        let authority = Authority {
            username: Some("username"),
            password: Some("password"),
            host: "example.com",
            port: Some("123"),
        };
        let path = None;
        let query = None;
        assert_eq!(Vec::<u8>::from(Uri {
            scheme,
            authority,
            path,
            query,
        }), b"https://username:password@example.com:123".to_vec());
    }

    #[test]
    fn from_str_invalid_uri_test() {
        assert_eq!(Uri::try_from("https://username:password@example.com:123/resource/subresource?parameter1=value1?parameter2=value2"), Err(HttpError::InvalidUri));
    }
}

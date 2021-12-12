extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Authority<'a> {
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub host: &'a str,
    pub port: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Authority<'a> {
    type Error = HttpError;

    fn try_from(mut src: &'a str) -> Result<Self, Self::Error> {
        let mut username_password_split = src.split("@");

        let (username, password) = if username_password_split.clone().count() == 2 {
            src = username_password_split.clone().nth(1).unwrap();

            let mut username_split = username_password_split.nth(0).unwrap().split(":");

            if username_split.clone().count() == 2 {
                (Some(username_split.nth(0).unwrap()), Some(username_split.nth(0).unwrap()))
            } else if username_split.clone().count() == 1 {
                (Some(username_split.nth(0).unwrap()), None)
            } else {
                return Err(HttpError::InvalidAuthority);
            }
        } else if username_password_split.count() == 1 {
            (None, None)
        } else {
            return Err(HttpError::InvalidAuthority);
        };

        let mut port_split = src.split(":");

        let port = if port_split.clone().count() == 2 {
            src = port_split.nth(0).unwrap();
            Some(port_split.nth(0).unwrap())
        } else if port_split.count() == 1 {
            None
        } else {
            return Err(HttpError::InvalidAuthority);
        };

        let host = src;

        Ok(Self {
            username,
            password,
            host,
            port,
        })
    }
}

impl<'a> From<Authority<'a>> for Vec<u8> {
    fn from(authority: Authority<'a>) -> Self {
        let mut data = vec![];

        if let Some(username) = authority.username {
            data.append(&mut username.as_bytes().to_vec());

            if let Some(password) = authority.password {
                data.append(&mut b":".to_vec());
                data.append(&mut password.as_bytes().to_vec());
            }

            data.append(&mut b"@".to_vec());
        }

        data.append(&mut authority.host.as_bytes().to_vec());

        if let Some(port) = authority.port {
            data.append(&mut b":".to_vec());
            data.append(&mut port.as_bytes().to_vec());
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_full_test() {
        let username = Some("username");
        let password = Some("password");
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Authority::try_from("username:password@example.com:123"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_full_test() {
        let username = Some("username");
        let password = Some("password");
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Vec::<u8>::from(Authority {
            username,
            password,
            host,
            port,
        }), b"username:password@example.com:123".to_vec());
    }

    #[test]
    fn from_str_no_password_test() {
        let username = Some("username");
        let password = None;
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Authority::try_from("username@example.com:123"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_no_password_test() {
        let username = Some("username");
        let password = None;
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Vec::<u8>::from(Authority {
            username,
            password,
            host,
            port,
        }), b"username@example.com:123".to_vec());
    }

    #[test]
    fn from_str_no_username_test() {
        let username = None;
        let password = None;
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Authority::try_from("example.com:123"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_no_username_test() {
        let username = None;
        let password = None;
        let host = "example.com";
        let port = Some("123");
        assert_eq!(Vec::<u8>::from(Authority {
            username,
            password,
            host,
            port,
        }), b"example.com:123".to_vec());
    }

    #[test]
    fn from_str_no_port_test() {
        let username = None;
        let password = None;
        let host = "example.com";
        let port = None;
        assert_eq!(Authority::try_from("example.com"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_no_port_test() {
        let username = None;
        let password = None;
        let host = "example.com";
        let port = None;
        assert_eq!(Vec::<u8>::from(Authority {
            username,
            password,
            host,
            port,
        }), b"example.com".to_vec());
    }

    #[test]
    fn from_str_invalid_authority_test() {
        assert_eq!(Authority::try_from("username@password@example.com:123"), Err(HttpError::InvalidAuthority));
    }
}

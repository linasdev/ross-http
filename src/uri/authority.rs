extern crate alloc;

use core::convert::{TryFrom, From};
use alloc::vec::Vec;

use crate::error::HttpError;

#[derive(Debug, PartialEq)]
pub struct Authority {
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<String>,
}

impl TryFrom<&str> for Authority {
    type Error = HttpError;

    fn try_from(mut src: &str) -> Result<Self, Self::Error> {
        let mut username_password_split = src.split("@");

        let (username, password) = if username_password_split.clone().count() == 2 {
            src = username_password_split.clone().nth(1).unwrap();

            let mut username_split = username_password_split.nth(0).unwrap().split(":");

            if username_split.clone().count() == 2 {
                (Some(username_split.nth(0).unwrap().to_string()), Some(username_split.nth(0).unwrap().to_string()))
            } else if username_split.clone().count() == 1 {
                (Some(username_split.nth(0).unwrap().to_string()), None)
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
            Some(port_split.nth(0).unwrap().to_string())
        } else if port_split.count() == 1 {
            None
        } else {
            return Err(HttpError::InvalidAuthority);
        };

        let host = src.to_string();

        Ok(Self {
            username,
            password,
            host,
            port,
        })
    }
}

impl From<Authority> for Vec<u8> {
    fn from(authority: Authority) -> Self {
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
        let username = Some("username".to_string());
        let password = Some("password".to_string());
        let host = "example.com".to_string();
        let port = Some("123".to_string());
        assert_eq!(Authority::try_from("username:password@example.com:123"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_full_test() {
        let username = Some("username".to_string());
        let password = Some("password".to_string());
        let host = "example.com".to_string();
        let port = Some("123".to_string());
        assert_eq!(Vec::<u8>::from(Authority {
            username,
            password,
            host,
            port,
        }), b"username:password@example.com:123".to_vec());
    }

    #[test]
    fn from_str_no_password_test() {
        let username = Some("username".to_string());
        let password = None;
        let host = "example.com".to_string();
        let port = Some("123".to_string());
        assert_eq!(Authority::try_from("username@example.com:123"), Ok(Authority {
            username,
            password,
            host,
            port,
        }));
    }

    #[test]
    fn to_bytes_no_password_test() {
        let username = Some("username".to_string());
        let password = None;
        let host = "example.com".to_string();
        let port = Some("123".to_string());
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
        let host = "example.com".to_string();
        let port = Some("123".to_string());
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
        let host = "example.com".to_string();
        let port = Some("123".to_string());
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
        let host = "example.com".to_string();
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
        let host = "example.com".to_string();
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

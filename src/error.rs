#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidMethod,
    InvalidScheme,
    InvalidAuthority,
    InvalidVersion,
    InvalidHeader,
}

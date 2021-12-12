#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidMethod,
    InvalidUri,
    InvalidScheme,
    InvalidAuthority,
    InvalidVersion,
    InvalidHeader,
}

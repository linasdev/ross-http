#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidMethod,
    InvalidUri,
    InvalidScheme,
    InvalidAuthority,
    InvalidQuery,
    InvalidVersion,
    InvalidHeader,
}

#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidMethod,
    InvalidAuthority,
    InvalidVersion,
    InvalidHeader,
}

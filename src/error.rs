#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidMethod,
    InvalidVersion,
    InvalidHeader,
}

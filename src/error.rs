#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidRequest,
    InvalidMethod,
    InvalidUri,
    InvalidScheme,
    InvalidAuthority,
    InvalidQuery,
    InvalidVersion,
    InvalidHeader,
    InvalidResponse,
    InvalidStatus,
    Exhausted,
}

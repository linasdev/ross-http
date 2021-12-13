#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

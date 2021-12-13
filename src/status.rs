extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::{From, TryFrom, TryInto};

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
    // 2xx
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,

    // 3xx
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl TryFrom<&str> for StatusCode {
    type Error = HttpError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let status_code = src.parse::<u32>().map_err(|_| HttpError::InvalidStatus)?;
        match status_code {
            // 1xx
            100 => Ok(StatusCode::Continue),
            101 => Ok(StatusCode::SwitchingProtocols),
            102 => Ok(StatusCode::Processing),

            // 2xx
            200 => Ok(StatusCode::Ok),
            201 => Ok(StatusCode::Created),
            202 => Ok(StatusCode::Accepted),
            203 => Ok(StatusCode::NonAuthoritativeInformation),
            204 => Ok(StatusCode::NoContent),
            205 => Ok(StatusCode::ResetContent),
            206 => Ok(StatusCode::PartialContent),
            207 => Ok(StatusCode::MultiStatus),
            208 => Ok(StatusCode::AlreadyReported),
            226 => Ok(StatusCode::ImUsed),

            // 3xx
            300 => Ok(StatusCode::MultipleChoices),
            301 => Ok(StatusCode::MovedPermanently),
            302 => Ok(StatusCode::Found),
            303 => Ok(StatusCode::SeeOther),
            304 => Ok(StatusCode::NotModified),
            305 => Ok(StatusCode::UseProxy),
            307 => Ok(StatusCode::TemporaryRedirect),
            308 => Ok(StatusCode::PermanentRedirect),

            // 4xx
            400 => Ok(StatusCode::BadRequest),
            401 => Ok(StatusCode::Unauthorized),
            402 => Ok(StatusCode::PaymentRequired),
            403 => Ok(StatusCode::Forbidden),
            404 => Ok(StatusCode::NotFound),
            405 => Ok(StatusCode::MethodNotAllowed),
            406 => Ok(StatusCode::NotAcceptable),
            407 => Ok(StatusCode::ProxyAuthenticationRequired),
            408 => Ok(StatusCode::RequestTimeout),
            409 => Ok(StatusCode::Conflict),
            410 => Ok(StatusCode::Gone),
            411 => Ok(StatusCode::LengthRequired),
            412 => Ok(StatusCode::PreconditionFailed),
            413 => Ok(StatusCode::PayloadTooLarge),
            414 => Ok(StatusCode::UriTooLong),
            415 => Ok(StatusCode::UnsupportedMediaType),
            416 => Ok(StatusCode::RangeNotSatisfiable),
            417 => Ok(StatusCode::ExpectationFailed),
            418 => Ok(StatusCode::ImATeapot),
            421 => Ok(StatusCode::MisdirectedRequest),
            422 => Ok(StatusCode::UnprocessableEntity),
            423 => Ok(StatusCode::Locked),
            424 => Ok(StatusCode::FailedDependency),
            426 => Ok(StatusCode::UpgradeRequired),
            428 => Ok(StatusCode::PreconditionRequired),
            429 => Ok(StatusCode::TooManyRequests),
            431 => Ok(StatusCode::RequestHeaderFieldsTooLarge),
            451 => Ok(StatusCode::UnavailableForLegalReasons),

            // 5xx
            500 => Ok(StatusCode::InternalServerError),
            501 => Ok(StatusCode::NotImplemented),
            502 => Ok(StatusCode::BadGateway),
            503 => Ok(StatusCode::ServiceUnavailable),
            504 => Ok(StatusCode::GatewayTimeout),
            505 => Ok(StatusCode::HttpVersionNotSupported),
            506 => Ok(StatusCode::VariantAlsoNegotiates),
            507 => Ok(StatusCode::InsufficientStorage),
            508 => Ok(StatusCode::LoopDetected),
            510 => Ok(StatusCode::NotExtended),
            511 => Ok(StatusCode::NetworkAuthenticationRequired),

            _ => Err(HttpError::InvalidStatus),
        }
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            // 1xx
            StatusCode::Continue => "100".to_string(),
            StatusCode::SwitchingProtocols => "101".to_string(),
            StatusCode::Processing => "102".to_string(),

            // 2xx
            StatusCode::Ok => "200".to_string(),
            StatusCode::Created => "201".to_string(),
            StatusCode::Accepted => "202".to_string(),
            StatusCode::NonAuthoritativeInformation => "203".to_string(),
            StatusCode::NoContent => "204".to_string(),
            StatusCode::ResetContent => "205".to_string(),
            StatusCode::PartialContent => "206".to_string(),
            StatusCode::MultiStatus => "207".to_string(),
            StatusCode::AlreadyReported => "208".to_string(),
            StatusCode::ImUsed => "226".to_string(),

            // 3xx
            StatusCode::MultipleChoices => "300".to_string(),
            StatusCode::MovedPermanently => "301".to_string(),
            StatusCode::Found => "302".to_string(),
            StatusCode::SeeOther => "303".to_string(),
            StatusCode::NotModified => "304".to_string(),
            StatusCode::UseProxy => "305".to_string(),
            StatusCode::TemporaryRedirect => "307".to_string(),
            StatusCode::PermanentRedirect => "308".to_string(),

            // 4xx
            StatusCode::BadRequest => "400".to_string(),
            StatusCode::Unauthorized => "401".to_string(),
            StatusCode::PaymentRequired => "402".to_string(),
            StatusCode::Forbidden => "403".to_string(),
            StatusCode::NotFound => "404".to_string(),
            StatusCode::MethodNotAllowed => "405".to_string(),
            StatusCode::NotAcceptable => "406".to_string(),
            StatusCode::ProxyAuthenticationRequired => "407".to_string(),
            StatusCode::RequestTimeout => "408".to_string(),
            StatusCode::Conflict => "409".to_string(),
            StatusCode::Gone => "410".to_string(),
            StatusCode::LengthRequired => "411".to_string(),
            StatusCode::PreconditionFailed => "412".to_string(),
            StatusCode::PayloadTooLarge => "413".to_string(),
            StatusCode::UriTooLong => "414".to_string(),
            StatusCode::UnsupportedMediaType => "415".to_string(),
            StatusCode::RangeNotSatisfiable => "416".to_string(),
            StatusCode::ExpectationFailed => "417".to_string(),
            StatusCode::ImATeapot => "418".to_string(),
            StatusCode::MisdirectedRequest => "412".to_string(),
            StatusCode::UnprocessableEntity => "422".to_string(),
            StatusCode::Locked => "423".to_string(),
            StatusCode::FailedDependency => "424".to_string(),
            StatusCode::UpgradeRequired => "426".to_string(),
            StatusCode::PreconditionRequired => "428".to_string(),
            StatusCode::TooManyRequests => "429".to_string(),
            StatusCode::RequestHeaderFieldsTooLarge => "431".to_string(),
            StatusCode::UnavailableForLegalReasons => "451".to_string(),

            // 5xx
            StatusCode::InternalServerError => "500".to_string(),
            StatusCode::NotImplemented => "501".to_string(),
            StatusCode::BadGateway => "502".to_string(),
            StatusCode::ServiceUnavailable => "503".to_string(),
            StatusCode::GatewayTimeout => "504".to_string(),
            StatusCode::HttpVersionNotSupported => "505".to_string(),
            StatusCode::VariantAlsoNegotiates => "506".to_string(),
            StatusCode::InsufficientStorage => "507".to_string(),
            StatusCode::LoopDetected => "508".to_string(),
            StatusCode::NotExtended => "510".to_string(),
            StatusCode::NetworkAuthenticationRequired => "511".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    pub code: StatusCode,
    pub reason: String,
}

impl From<StatusCode> for Status {
    fn from(status_code: StatusCode) -> Self {
        let (code, reason) = match status_code {
            // 1xx
            StatusCode::Continue => (status_code, "Continue".to_string()),
            StatusCode::SwitchingProtocols => (status_code, "Switching Protocols".to_string()),
            StatusCode::Processing => (status_code, "Processing".to_string()),

            // 2xx
            StatusCode::Ok => (status_code, "Ok".to_string()),
            StatusCode::Created => (status_code, "Created".to_string()),
            StatusCode::Accepted => (status_code, "Accepted".to_string()),
            StatusCode::NonAuthoritativeInformation => {
                (status_code, "Non Authoritative Information".to_string())
            }
            StatusCode::NoContent => (status_code, "No Content".to_string()),
            StatusCode::ResetContent => (status_code, "Reset Content".to_string()),
            StatusCode::PartialContent => (status_code, "Partial Content".to_string()),
            StatusCode::MultiStatus => (status_code, "Multi Status".to_string()),
            StatusCode::AlreadyReported => (status_code, "Already Reported".to_string()),
            StatusCode::ImUsed => (status_code, "Im Used".to_string()),

            // 3xx
            StatusCode::MultipleChoices => (status_code, "Multiple Choices".to_string()),
            StatusCode::MovedPermanently => (status_code, "Moved Permanently".to_string()),
            StatusCode::Found => (status_code, "Found".to_string()),
            StatusCode::SeeOther => (status_code, "See Other".to_string()),
            StatusCode::NotModified => (status_code, "Not Modified".to_string()),
            StatusCode::UseProxy => (status_code, "Use Proxy".to_string()),
            StatusCode::TemporaryRedirect => (status_code, "Temporary Redirect".to_string()),
            StatusCode::PermanentRedirect => (status_code, "Permanent Redirect".to_string()),

            // 4xx
            StatusCode::BadRequest => (status_code, "Bad Request".to_string()),
            StatusCode::Unauthorized => (status_code, "Unauthorized".to_string()),
            StatusCode::PaymentRequired => (status_code, "Payment Required".to_string()),
            StatusCode::Forbidden => (status_code, "Forbidden".to_string()),
            StatusCode::NotFound => (status_code, "Not Found".to_string()),
            StatusCode::MethodNotAllowed => (status_code, "Method Not Allowed".to_string()),
            StatusCode::NotAcceptable => (status_code, "Not Acceptable".to_string()),
            StatusCode::ProxyAuthenticationRequired => {
                (status_code, "Proxy Authentication Required".to_string())
            }
            StatusCode::RequestTimeout => (status_code, "Request Timeout".to_string()),
            StatusCode::Conflict => (status_code, "Conflict".to_string()),
            StatusCode::Gone => (status_code, "Gone".to_string()),
            StatusCode::LengthRequired => (status_code, "Length Required".to_string()),
            StatusCode::PreconditionFailed => (status_code, "Precondition Failed".to_string()),
            StatusCode::PayloadTooLarge => (status_code, "Payload Too Large".to_string()),
            StatusCode::UriTooLong => (status_code, "Uri Too Long".to_string()),
            StatusCode::UnsupportedMediaType => (status_code, "Unsupported Media Type".to_string()),
            StatusCode::RangeNotSatisfiable => (status_code, "Range Not Satisfiable".to_string()),
            StatusCode::ExpectationFailed => (status_code, "Expectation Failed".to_string()),
            StatusCode::ImATeapot => (status_code, "Im A Teapot".to_string()),
            StatusCode::MisdirectedRequest => (status_code, "Misdirected Request".to_string()),
            StatusCode::UnprocessableEntity => (status_code, "Unprocessable Entity".to_string()),
            StatusCode::Locked => (status_code, "Locked".to_string()),
            StatusCode::FailedDependency => (status_code, "Failed Dependency".to_string()),
            StatusCode::UpgradeRequired => (status_code, "Upgrade Required".to_string()),
            StatusCode::PreconditionRequired => (status_code, "Precondition Required".to_string()),
            StatusCode::TooManyRequests => (status_code, "Too Many Requests".to_string()),
            StatusCode::RequestHeaderFieldsTooLarge => {
                (status_code, "Request Header Fields Too Large".to_string())
            }
            StatusCode::UnavailableForLegalReasons => {
                (status_code, "Unavailable For Legal Reasons".to_string())
            }

            // 5xx
            StatusCode::InternalServerError => (status_code, "Internal Server Error".to_string()),
            StatusCode::NotImplemented => (status_code, "Not Implemented".to_string()),
            StatusCode::BadGateway => (status_code, "Bad Gateway".to_string()),
            StatusCode::ServiceUnavailable => (status_code, "Service Unavailable".to_string()),
            StatusCode::GatewayTimeout => (status_code, "Gateway Timeout".to_string()),
            StatusCode::HttpVersionNotSupported => {
                (status_code, "Http Version Not Supported".to_string())
            }
            StatusCode::VariantAlsoNegotiates => {
                (status_code, "Variant Also Negotiates".to_string())
            }
            StatusCode::InsufficientStorage => (status_code, "Insufficient Storage".to_string()),
            StatusCode::LoopDetected => (status_code, "Loop Detected".to_string()),
            StatusCode::NotExtended => (status_code, "Not Extended".to_string()),
            StatusCode::NetworkAuthenticationRequired => {
                (status_code, "Network Authentication Required".to_string())
            }
        };

        Self { code, reason }
    }
}

impl TryFrom<&str> for Status {
    type Error = HttpError;

    fn try_from(mut src: &str) -> Result<Self, Self::Error> {
        let code = if let Some(index) = src.find(" ") {
            let method_split = src.split_at(index);

            src = &method_split.1[1..];

            method_split.0.try_into()?
        } else {
            return Err(HttpError::Exhausted);
        };

        let reason = src.to_string();

        Ok(Self { code, reason })
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let mut data = String::new();

        data += self.code.to_string().as_str();
        data += " ";
        data += self.reason.as_str();

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_status_code_ok_test() {
        assert_eq!(
            Status::try_from("200 Ok"),
            Ok(Status {
                code: StatusCode::Ok,
                reason: "Ok".to_string(),
            })
        );
    }

    #[test]
    fn to_string_status_code_ok_test() {
        assert_eq!(
            Status {
                code: StatusCode::Ok,
                reason: "Ok".to_string(),
            }
            .to_string(),
            "200 Ok".to_string(),
        );
    }

    #[test]
    fn from_str_status_code_no_content_test() {
        assert_eq!(
            Status::try_from("204 No Content"),
            Ok(Status {
                code: StatusCode::NoContent,
                reason: "No Content".to_string(),
            })
        );
    }

    #[test]
    fn to_string_status_code_no_content_test() {
        assert_eq!(
            Status {
                code: StatusCode::NoContent,
                reason: "No Content".to_string(),
            }
            .to_string(),
            "204 No Content".to_string(),
        );
    }

    #[test]
    fn from_status_code_ok_test() {
        assert_eq!(
            Status::from(StatusCode::Ok),
            Status {
                code: StatusCode::Ok,
                reason: "Ok".to_string(),
            }
        );
    }

    #[test]
    fn from_status_code_no_content_test() {
        assert_eq!(
            Status::from(StatusCode::NoContent),
            Status {
                code: StatusCode::NoContent,
                reason: "No Content".to_string(),
            }
        );
    }
}

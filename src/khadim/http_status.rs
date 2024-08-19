#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
pub enum HttpStatus {
    // 1xx: Informational
    Continue,                    // 100
    SwitchingProtocols,          // 101

    // 2xx: Success
    Ok,                          // 200
    Created,                     // 201
    Accepted,                    // 202
    NonAuthoritativeInformation, // 203
    NoContent,                   // 204
    ResetContent,                // 205
    PartialContent,              // 206

    // 3xx: Redirection
    MultipleChoices,             // 300
    MovedPermanently,            // 301
    Found,                       // 302
    SeeOther,                    // 303
    NotModified,                 // 304
    UseProxy,                    // 305
    TemporaryRedirect,           // 307
    PermanentRedirect,           // 308

    // 4xx: Client Error
    BadRequest,                  // 400
    Unauthorized,                // 401
    PaymentRequired,             // 402
    Forbidden,                   // 403
    NotFound,                    // 404
    MethodNotAllowed,            // 405
    NotAcceptable,               // 406
    ProxyAuthenticationRequired, // 407
    RequestTimeout,              // 408
    Conflict,                    // 409
    Gone,                        // 410
    LengthRequired,              // 411
    PreconditionFailed,          // 412
    PayloadTooLarge,             // 413
    UriTooLong,                  // 414
    UnsupportedMediaType,        // 415
    RangeNotSatisfiable,         // 416
    ExpectationFailed,           // 417
    ImATeapot,                   // 418
    UnprocessableEntity,         // 422
    TooManyRequests,             // 429

    // 5xx: Server Error
    InternalServerError,         // 500
    NotImplemented,              // 501
    BadGateway,                  // 502
    ServiceUnavailable,          // 503
    GatewayTimeout,              // 504
    HttpVersionNotSupported,     // 505
}

impl HttpStatus {
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let response = match self {
            HttpStatus::Continue => "100 Continue",
            HttpStatus::SwitchingProtocols => "101 Switching Protocols",
            HttpStatus::Ok => "200 OK",
            HttpStatus::Created => "201 Created",
            HttpStatus::Accepted => "202 Accepted",
            HttpStatus::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            HttpStatus::NoContent => "204 No Content",
            HttpStatus::ResetContent => "205 Reset Content",
            HttpStatus::PartialContent => "206 Partial Content",
            HttpStatus::MultipleChoices => "300 Multiple Choices",
            HttpStatus::MovedPermanently => "301 Moved Permanently",
            HttpStatus::Found => "302 Found",
            HttpStatus::SeeOther => "303 See Other",
            HttpStatus::NotModified => "304 Not Modified",
            HttpStatus::UseProxy => "305 Use Proxy",
            HttpStatus::TemporaryRedirect => "307 Temporary Redirect",
            HttpStatus::PermanentRedirect => "308 Permanent Redirect",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Unauthorized => "401 Unauthorized",
            HttpStatus::PaymentRequired => "402 Payment Required",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::MethodNotAllowed => "405 Method Not Allowed",
            HttpStatus::NotAcceptable => "406 Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            HttpStatus::RequestTimeout => "408 Request Timeout",
            HttpStatus::Conflict => "409 Conflict",
            HttpStatus::Gone => "410 Gone",
            HttpStatus::LengthRequired => "411 Length Required",
            HttpStatus::PreconditionFailed => "412 Precondition Failed",
            HttpStatus::PayloadTooLarge => "413 Payload Too Large",
            HttpStatus::UriTooLong => "414 URI Too Long",
            HttpStatus::UnsupportedMediaType => "415 Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "416 Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "417 Expectation Failed",
            HttpStatus::ImATeapot => "418 I'm a teapot",
            HttpStatus::UnprocessableEntity => "422 Unprocessable Entity",
            HttpStatus::TooManyRequests => "429 Too Many Requests",
            HttpStatus::InternalServerError => "500 Internal Server Error",
            HttpStatus::NotImplemented => "501 Not Implemented",
            HttpStatus::BadGateway => "502 Bad Gateway",
            HttpStatus::ServiceUnavailable => "503 Service Unavailable",
            HttpStatus::GatewayTimeout => "504 Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "505 HTTP Version Not Supported",
        };
        write!(f, "{}", response)
    }
}
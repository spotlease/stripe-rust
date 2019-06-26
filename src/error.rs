use serde::Deserialize;
use std::io;
use reqwest;
use serde_json;
use serde_qs;
use failure::{Fail, SyncFailure};

/// An error encountered when communicating with the Stripe API.
#[derive(Debug, Fail)]
pub enum Error {
    /// An error reported by Stripe.
    #[fail(display = "error reported by stripe: {:#?}", _0)]
    Stripe(RequestError),
    /// A networking error communicating with the Stripe server.
    #[fail(display = "error communicating with stripe: {}", _0)]
    Http(#[cause] reqwest::Error),
    /// Error serializing form body.
    #[fail(display = "error serializing form body: {}", _0)]
    FormSerialization(#[cause] failure::Error),
    //TODO: doc
    #[fail(display = "error parsing url: {}", _0)]
    Url(#[cause] reqwest::UrlError),
    /// An error reading the response body.
    #[fail(display = "error reading response from stripe: {}", _0)]
    Io(#[cause] io::Error),
    /// An error converting between wire format and Rust types.
    #[fail(display = "error converting between wire format and Rust types: {}", _0)]
    Conversion(#[cause] failure::Error),
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Stripe(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_qs::Error> for Error {
    fn from(err: serde_qs::Error) -> Error {
        Error::FormSerialization(SyncFailure::new(err).into())
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Error {
        Error::Url(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Conversion(SyncFailure::new(err).into())
    }
}


/// The list of possible values for a RequestError's type.
#[derive(Debug, PartialEq, Deserialize)]
pub enum ErrorType {
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "idempotency_error")]
    Idempotency,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,
    #[serde(rename = "validation_error")]
    Validation,
}

/// The list of possible values for a RequestError's code.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidNumber,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidCvc,
    InvalidSwipeData,
    IncorrectNumber,
    ExpiredCard,
    IncorrectCvc,
    IncorrectZip,
    CardDeclined,
    Missing,
    ProcessingError,
}

/// An error reported by stripe in a request's response.
///
/// For more details see https://stripe.com/docs/api#errors.
#[derive(Debug, Deserialize)]
pub struct RequestError {
    /// The HTTP status in the response.
    #[serde(skip_deserializing)]
    pub http_status: u16,

    /// The type of error returned.
    #[serde(rename = "type")]
    pub error_type: ErrorType,

    /// A human-readable message providing more details about the error.
    /// For card errors, these messages can be shown to end users.
    #[serde(default)]
    pub message: Option<String>,

    /// For card errors, a value describing the kind of card error that occured.
    pub code: Option<ErrorCode>,

    /// For card errors resulting from a bank decline, a string indicating the
    /// bank's reason for the decline if they provide one.
    pub decline_code: Option<String>,

    /// The ID of the failed charge, if applicable.
    pub charge: Option<String>,
}
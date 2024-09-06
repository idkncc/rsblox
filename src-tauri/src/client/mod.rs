#![allow(dead_code)]

mod client;
mod validation;

pub mod discovery;
pub mod economy;
pub mod friends;
pub mod games;
pub mod presence;
pub mod search;
pub mod thumbnails;
pub mod users;

pub use client::RobloxApi;
use economy::PurchaseTradableLimitedError;
use serde::{Deserialize, Serialize};

// Used in request header keys.
const XCSRF_HEADER: &str = "x-csrf-token";

// The user agent used for fussy endpoints.
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0";

// The content type used for fussy endpoints.
const CONTENT_TYPE: &str = "application/json;charset=utf-8";

/// The universal errors
#[non_exhaustive]
#[derive(thiserror::Error, Debug, Default)]
pub enum RobloxError {
    /// Used when an endpoint returns status code 429.
    #[default]
    #[error("Too Many Requests")]
    TooManyRequests,

    /// Used when an endpoint returns status code 500.
    #[error("Internal Server Error")]
    InternalServerError,

    /// Used when an endpoint returns status code 400 and does not embed an error.
    /// This is used when the server cannot process the data sent, whether
    /// it be because it is in the wrong format or it contains too much data.
    #[error("Bad Request")]
    BadRequest,

    /// Returned when the user does not have a valid roblosecurity, or
    /// does not have authorization to access the endpoint.
    ///
    /// This is also used as the backup error when an endpoint returns a 401 status code
    /// but the error cannot be parsed from the response.
    ///
    /// Roblox error code 0.
    #[error("Invalid Roblosecurity")]
    InvalidRoblosecurity,

    /// Returned when the endpoint returns a 401 status code, but the error response
    /// contains an unknown Roblox error code.
    #[error("Unknown Roblox Error Code {code}: {message}")]
    UnknownRobloxErrorCode {
        /// The error code (not status code) returned by roblox.
        code: u16,
        /// The error message returned by roblox.
        message: String,
    },

    /// Used when no roblosecurity is set, on an endpoint that requires it.
    #[error("Roblosecurity Not Set")]
    RoblosecurityNotSet,

    /// Used for any status codes that do not fit any enum variants of this error.
    /// If you encounter this enum variant, please submit an issue so a variant can be
    /// made or the crate can be fixed.
    #[error("Unidentified Status Code {0}")]
    UnidentifiedStatusCode(u16),

    /// Used when the response from an API endpoint is malformed.
    #[error("Malformed Response. If this occurs often it may be a bug. Please report it to the issues page."
    )]
    MalformedResponse,

    /// Used when an endpoint rejects a request due to an invalid xcsrf.
    /// Mostly used internally invalid xcsrf is returned due to the fact that rust does not
    /// allow async recursion without making a type signature extremely messy.
    #[error("Invalid Xcsrf. New Xcsrf Contained In Error.")]
    InvalidXcsrf(String),

    /// Used when an endpoint returns a 403 status code, doesn't need a challenge, but the response does not contain
    /// a new xcsrf.
    #[error("Missing Xcsrf")]
    XcsrfNotReturned,

    /// Used when an endpoint returns a 403 status code, but not because of an invalid xcsrf.
    /// The string inside this error variant is a challenge id, which can be used to complete the challenge
    /// (which can be either a captcha or a two step verification code).
    #[error("Challenge Required. A captcha or two step authentication must be completed using challenge id {0}."
    )]
    ChallengeRequired(String),

    /// Used when an endpoint returns a 403 status code, can be parsed into a roblox error,
    /// but the error message is incorrect or the challenge id is not returned. This also means that no xcsrf was returned.
    #[error("Unknown Status Code 403 Format. If this occurs often it may be a bug. Please report it to the issues page."
    )]
    UnknownStatus403Format,

    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_tradable_limited`].
    #[error("{0}")]
    PurchaseTradableLimitedError(PurchaseTradableLimitedError),

    // /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_non_tradable_limited`].
    // #[error("{0}")]
    // PurchaseNonTradableLimitedError(PurchaseNonTradableLimitedError),
    /// Used for any reqwest error that occurs.
    #[error("RequestError {0}")]
    ReqwestError(reqwest::Error),

    /// Used when an io error occurs.
    #[error("IoError {0}")]
    IoError(#[from] std::io::Error),

    /// Used when a file system path passed to a method is invalid.
    #[error("Invalid Path {0}")]
    InvalidPath(String),
}

/// The universal struct for a Roblox user.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}
/// The maximum amount of instances to return from an endpoint. Used as a parameter in various methods that call
/// endpoints.
///
/// This is an enum instead of an integer as these are usually the only values that are accepted by Roblox
/// for the limit parameter.
///
/// This is the most common limit used on Roblox endpoints. However, not all endpoints use this limit.
/// Some alternative limits are as follows:
/// * [`catalog::CatalogQueryLimit`]
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum Limit {
    #[default]
    Ten,
    TwentyFive,
    Fifty,
    Hundred,
}

impl Limit {
    fn to_u64(self) -> u64 {
        match self {
            Limit::Ten => 10,
            Limit::TwentyFive => 25,
            Limit::Fifty => 50,
            Limit::Hundred => 100,
        }
    }
}

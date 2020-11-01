use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// ErrorCode for BBB API
pub enum ErrorCode {
    /// Success Response from BBB API
    SUCCESS,

    /// Failed Response from BBB API
    FAILED,
}

#[derive(Debug, Deserialize)]
/// Custom error type for API Requests
pub struct BBBError {
    #[serde(rename = "returncode")]
    /// Return code for [ErrorCode]
    return_code: ErrorCode,

    /// Error Message Key
    #[serde(rename = "messageKey")]
    message_key: String,

    /// Error Message
    message: String,
}

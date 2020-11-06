use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// ErrorCode for BBB API
pub enum ResponseCode {
    /// Success Response from BBB API
    SUCCESS,

    /// Failed Response from BBB API
    FAILED,
}

#[derive(Debug, Deserialize)]
/// Custom error type for API Requests
pub struct BBBError {
    #[serde(rename = "returncode")]
    /// Return code for [ResponseCode]
    pub return_code: ResponseCode,

    /// Error Message Key
    #[serde(rename = "messageKey")]
    pub message_key: String,

    /// Error Message
    pub message: String,
}

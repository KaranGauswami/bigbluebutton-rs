use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum ErrorCode {
    SUCCESS,
    FAILED,
}

#[derive(Debug, Deserialize)]
pub struct BBBError {
    #[serde(rename = "returncode")]
    return_code: ErrorCode,

    #[serde(rename = "messageKey")]
    message_key: String,

    message: String,
}

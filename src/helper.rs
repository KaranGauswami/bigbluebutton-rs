use crate::Bigbluebutton;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

impl Bigbluebutton {
    /// hash function for converting to SHA-256
    pub(crate) fn hash(payload: Vec<String>) -> String {
        let mut hasher = Sha256::new();

        hasher.update(payload.join(""));
        let result = hasher.finalize();
        let hash_value = result.as_slice();

        hex::encode(hash_value)
    }

    /// serialize query parameters into query string
    pub(crate) fn serialize_params(params: HashMap<String, String>) -> String {
        let mut key_value = Vec::new();
        for (key, value) in params {
            key_value.push(format!("{}={}", key, value));
        }
        let serialized = key_value.join("&");
        serialized
    }
}

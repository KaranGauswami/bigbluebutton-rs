use crate::Bigbluebutton;
use sha2::{Digest, Sha256};

impl Bigbluebutton {
    /// hash function for converting to SHA-256
    pub(crate) fn hash(payload: Vec<&str>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload.join(""));
        let result = hasher.finalize();
        let hash_value = result.as_slice();
        hex::encode(hash_value)
    }

    /// serialize query parameters into query string
    pub(crate) fn serialize_params(params: Vec<(&str, &str)>) -> String {
        let collection: Vec<String> = params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();
        collection.join("&")
    }
    #[cfg(feature = "webhook")]
    pub(crate) fn hash_unsecure(payload: Vec<&str>) -> String {
        let mut hasher = sha1::Sha1::new();
        hasher.update(payload.join(""));
        let result = hasher.finalize();
        let hash_value = result.as_slice();
        hex::encode(result)
    }
}
pub(crate) trait GetApiName {
    fn get_api_name(&self) -> &str;
}

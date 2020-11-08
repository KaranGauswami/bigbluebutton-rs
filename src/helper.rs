use crate::Bigbluebutton;

use sha2::Digest;

impl Bigbluebutton {
    /// hash function for converting to SHA-256
    #[cfg(not(feature = "webhook"))]
    pub(crate) fn hash(payload: Vec<&str>) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(payload.join(""));
        let result = hasher.finalize();
        let hash_value = result.as_slice();
        hex::encode(hash_value)
    }
    #[cfg(feature = "webhook")]
    pub(crate) fn hash(payload: Vec<&str>) -> String {
        let mut hasher = sha1::Sha1::new();
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
}
pub(crate) trait GetApiName {
    fn get_api_name(&self) -> &str;
}

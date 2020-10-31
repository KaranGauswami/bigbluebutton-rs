// #![deny(missing_docs, rust_2018_idioms)]
//! BigBlueButton is an open source web conferencing system for online learning.
//!
//! This crate provides interface for interacting with Bigbluebutton APIs.
//!
//! More details can be found [here](https://docs.bigbluebutton.org/dev/api.html)
//! # Examples
//!```rust
//! # use bigbluebutton::Bigbluebutton;
//!
//! // Creates new BBB Instance
//! let bbb = Bigbluebutton::new(
//!     "https://example.com/bigbluebutton/",
//!     "BBBSECRET",
//!  );
//!
//! let params = vec![
//!     ("password", "pass"),
//!     ("fullName", "name"),
//!     ("meetingId", "1"),
//! ];
//!
//! let url = bbb.generate_url("join", params);
//! # assert_eq!(url,"https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingId=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf");
//! println!("{}",url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingId=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf
//! ```

/// Error Module
pub mod error;
mod helper;
mod resources;

use async_trait::async_trait;
use helper::GetApiName;
pub use resources::administration;
pub use resources::monitoring;

/// Implementation of Bigbluebutton APIs
pub struct Bigbluebutton {
    salt: String,
    url: String,
}
impl Bigbluebutton {
    /// creates new BBB API Client
    pub fn new<'a>(url: &'a str, salt: &'a str) -> Self {
        let mut new_url = url.to_string();
        new_url.push_str("api/");
        Self {
            salt: salt.to_string(),
            url: new_url,
        }
    }

    fn create_api_url<T>(&self, request: &T) -> String
    where
        T: serde::Serialize + GetApiName,
    {
        let action = request.get_api_name();
        let query_params = serde_qs::to_string(request).unwrap();
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, &self.salt]);
        format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        )
    }

    /// Generates BBB URL with checksum to interact with BBB server
    pub fn generate_url(&self, action: &str, params: Vec<(&str, &str)>) -> String {
        let query_params = self::Bigbluebutton::serialize_params(params);
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, &self.salt]);
        format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        )
    }

    pub(crate) async fn dispatch<'a, R, T>(&self, request: &R) -> Result<T, self::error::BBBError>
    where
        R: GetApiName + serde::Serialize,
        T: serde::Deserialize<'a>,
    {
        let url = self.create_api_url(request);
        let text_response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let return_response;
        if text_response.contains("SUCCESS") {
            return_response = Ok(serde_xml_rs::from_str::<T>(&text_response).unwrap());
        } else {
            return_response =
                Err(serde_xml_rs::from_str::<self::error::BBBError>(&text_response).unwrap());
        }
        return_response
    }
}

#[async_trait]
pub trait Execute<T, E> {
    async fn execute(&self, request: &T) -> Result<E, error::BBBError>;
}

#[cfg(test)]
mod test {
    use super::Bigbluebutton;

    #[test]
    fn generate_url() {
        let bbb = Bigbluebutton::new("https://example.com/bigbluebutton/", "yourbbbsecret");

        let params = vec![
            ("password", "yourpassword"),
            ("fullName", "fullname"),
            ("meetingID", "127"),
        ];

        let url = bbb.generate_url("join", params);
        assert_eq!(url,"https://example.com/bigbluebutton/api/join?password=yourpassword&fullName=fullname&meetingID=127&checksum=40799bd174680c4613a512cb000eee0bb9beafe1bcb1d380b23c0720e5d0f609");
    }
}

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

mod helper;
pub mod meeting;

use meeting::GetApiName;
use reqwest;
use serde::Deserialize;

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

    /// API Executer
    pub async fn execute<T>(self, request: &T) -> Result<(), reqwest::Error>
    where
        T: serde::Serialize + GetApiName,
    {
        let action = request.get_query_params();
        let query_params = serde_qs::to_string(request).unwrap();
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, &self.salt]);
        let url = format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        );
        let response = reqwest::get(&url).await?.text().await?;
        Ok(())
    }

    /// Generates BBB URL with checksum to interact with BBB server
    pub fn generate_url(self, action: &str, params: Vec<(&str, &str)>) -> String {
        let query_params = self::Bigbluebutton::serialize_params(params);
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, &self.salt]);
        format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        )
    }
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

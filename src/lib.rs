// #![deny(missing_docs, rust_2018_idioms)]
//! BigBlueButton is an open source web conferencing system for online learning.
//!
//! This crate provides interface for interacting with Bigbluebutton APIs.
//!
//! More details can be found [here](https://docs.bigbluebutton.org/dev/api.html)
//! # Examples
//! - Generating URLs
//!```rust
//! # use bigbluebutton::Bigbluebutton;
//!
//! // Creates new BBB Instance
//! let client = Bigbluebutton::new(
//!     "https://example.com/bigbluebutton/",
//!     "BBBSECRET",
//!  );
//!
//! let params = vec![
//!     ("password", "pass"),
//!     ("fullName", "name"),
//!     ("meetingID", "1"),
//! ];
//!
//! let url =client.generate_url("join", params).expect("Unable to generate url");
//! println!("{}",url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingID=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf
//! ```
//! - Creating Meeting
//! ```rust,no_run
//! use bigbluebutton::Bigbluebutton;
//! use bigbluebutton::administration::CreateMeetingRequest;
//!
//! #[tokio::main]
//! async fn main(){
//! # let client = Bigbluebutton::new(
//!     "https://example.com/bigbluebutton/",
//!     "BBBSECRET",
//!  );
//!  let mut request = CreateMeetingRequest::new("12");
//! client.create_meeting(&request).await;
//! }
//! ```

#[doc(hidden)]
pub mod error;
mod helper;
mod resources;

#[cfg(feature = "administration")]
pub use resources::administration;

#[cfg(feature = "monitoring")]
pub use resources::monitoring;

#[cfg(feature = "webhook")]
pub use resources::webhook;

#[cfg(feature = "recording")]
pub use resources::recording;

/// Implementation of Bigbluebutton APIs
#[derive(Debug, Clone)]
pub struct Bigbluebutton {
    salt: String,
    url: String,
}
impl Bigbluebutton {
    /// creates new BBB API Client
    pub fn new(url: impl ToString, salt: impl ToString) -> Self {
        let new_url = format!("{}api/", url.to_string());
        Self {
            salt: salt.to_string(),
            url: new_url,
        }
    }

    fn create_api_url<T>(&self, api_path: &str, request: &T) -> anyhow::Result<String>
    where
        T: serde::Serialize,
    {
        let query_params = serde_qs::to_string(request)?;
        let checksum = self::Bigbluebutton::hash(vec![api_path, &query_params, &self.salt]);
        Ok(format!(
            "{}{}?{}&checksum={}",
            self.url, api_path, query_params, checksum
        ))
    }

    /// Generates BBB URL with checksum to interact with BBB server
    pub fn generate_url<K, V>(
        &self,
        api_path: &str,
        params: impl IntoIterator<Item = (K, V)>,
    ) -> Result<String, url::ParseError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = reqwest::Url::parse_with_params(&self.url, params)?;
        let query = url.query().expect("Query params not found");
        // let query_params = self::Bigbluebutton::serialize_params(params);
        let checksum = self::Bigbluebutton::hash(vec![api_path, &query, &self.salt]);
        Ok(format!("{}&checksum={}", url, checksum))
    }

    pub(crate) async fn dispatch<'a, R, T>(&self, api_path: &str, request: &R) -> anyhow::Result<T>
    where
        R: serde::Serialize,
        T: serde::Deserialize<'a>,
    {
        let url = self.create_api_url(api_path, request)?;
        let text_response = reqwest::get(&url).await?.text().await?;
        if text_response.contains("SUCCESS") {
            Ok(serde_xml_rs::from_str::<T>(&text_response)?)
        } else {
            let error = serde_xml_rs::from_str::<self::error::BBBError>(&text_response)?;
            Err(anyhow::anyhow!("{}", error.message))
        }
    }
}

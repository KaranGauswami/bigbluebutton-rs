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
//! println!("{}",url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingId=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf
//! ```
//! - Creating Meeting
//! ```rust,no_run
//! use bigbluebutton::{Bigbluebutton,Execute};
//! use bigbluebutton::administration::CreateMeetingRequest;
//!
//! #[tokio::main]
//! async fn main(){
//! # let bbb = Bigbluebutton::new(
//!     "https://example.com/bigbluebutton/",
//!     "BBBSECRET",
//!  );
//!  let mut request = CreateMeetingRequest::new("12");
//!  bbb.execute(&request).await;
//! }
//! ```

/// Error Module
pub mod error;
mod helper;
mod resources;

use async_trait::async_trait;
use helper::GetApiName;
#[cfg(feature = "administration")]
pub use resources::administration;

#[cfg(feature = "monitoring")]
pub use resources::monitoring;

#[cfg(feature = "webhook")]
pub use resources::webhook;

#[cfg(feature = "recording")]
pub use resources::recording;

/// Implementation of Bigbluebutton APIs
pub struct Bigbluebutton {
    salt: String,
    url: String,
}
impl Bigbluebutton {
    /// creates new BBB API Client
    pub fn new(url: impl ToString, salt: impl ToString) -> Self {
        let mut new_url = url.to_string();
        new_url.push_str("api/");
        Self {
            salt: salt.to_string(),
            url: new_url,
        }
    }

    fn create_api_url<T>(&self, request: &T) -> anyhow::Result<String>
    where
        T: serde::Serialize + GetApiName,
    {
        let action = request.get_api_name();
        let query_params = serde_qs::to_string(request)?;
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, &self.salt]);
        Ok(format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        ))
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

    pub(crate) async fn dispatch<'a, R, T>(&self, request: &R) -> anyhow::Result<T>
    where
        R: GetApiName + serde::Serialize,
        T: serde::Deserialize<'a>,
    {
        let url = self.create_api_url(request)?;
        eprintln!("{}", url);
        let text_response = reqwest::get(&url).await?.text().await?;
        eprintln!("{:?}", text_response);
        let return_response;
        if text_response.contains("SUCCESS") {
            return_response = Ok(serde_xml_rs::from_str::<T>(&text_response)?);
        } else {
            let error = serde_xml_rs::from_str::<self::error::BBBError>(&text_response)?;
            return_response = Err(anyhow::anyhow!("{}", error.message));
        }
        return_response
    }
}

#[async_trait]
/// execute trait method for executing API requests
pub trait Execute<T, E> {
    /// trait function to execute requests
    async fn execute(&self, request: &T) -> anyhow::Result<E>;
}

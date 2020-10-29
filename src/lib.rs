#![deny(missing_docs, rust_2018_idioms)]
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
//!     "https://example.com/bigbluebutton/api/",
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
//!```
mod helper;
// mod meeting;

/// Implementation of Bigbluebutton APIs
pub struct Bigbluebutton<'a> {
    salt: &'a str,
    url: &'a str,
}
impl<'a> Bigbluebutton<'a> {
    /// creates new BBB API Client
    pub fn new(url: &'a str, salt: &'a str) -> Self {
        Self { salt, url }
    }

    /// Generates BBB URL with checksum to interact with BBB server
    pub fn generate_url(self, action: &str, params: Vec<(&str, &str)>) -> String {
        let query_params = self::Bigbluebutton::serialize_params(params);
        let checksum = self::Bigbluebutton::hash(vec![action, &query_params, self.salt]);
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
        let bbb = Bigbluebutton::new("https://example.com/bigbluebutton/api/", "yourbbbsecret");

        let params = vec![
            ("password", "yourpassword"),
            ("fullName", "fullname"),
            ("meetingID", "127"),
        ];

        let url = bbb.generate_url("join", params);
        assert_eq!(url,"https://example.com/bigbluebutton/api/join?password=yourpassword&fullName=fullname&meetingID=127&checksum=40799bd174680c4613a512cb000eee0bb9beafe1bcb1d380b23c0720e5d0f609");
    }
}

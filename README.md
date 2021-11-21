# bigbluebutton-rs

[![Crates.io](https://img.shields.io/crates/v/bigbluebutton.svg)](https://crates.io/crates/bigbluebutton)
[![Documentation](https://docs.rs/bigbluebutton/badge.svg)](https://docs.rs/bigbluebutton/)
[![build](https://github.com/KaranGauswami/bigbluebutton-rs/workflows/Rust/badge.svg)](https://github.com/KaranGauswami/bigbluebutton-rs/actions)

Rust crate for interacting with BBB APIs.

BigBlueButton is an open source web conferencing system for online learning.

This crate provides an interface for interacting with Bigbluebutton APIs.
More details can be found [here](https://docs.bigbluebutton.org/dev/api.html).

# Examples

## Gerenating API urls

```rust
use bigbluebutton::Bigbluebutton;

// Creates new BBB Instance
let client = Bigbluebutton::new(
    "https://example.com/bigbluebutton/",
    "BBBSECRET",
  );

let params = vec![
    ("password", "pass"),
    ("fullName", "name"),
    ("meetingID", "1"),
  ];

let url = bbb.generate_url("join", params);

println!("{}",url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingID=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf
```

## Creating Meeting

```rust
// creaing meeting
use bigbluebutton::administration::CreateMeetingRequest;
use bigbluebutton::{Bigbluebutton, Execute};

#[tokio::main]
async fn main() {
    let client = Bigbluebutton::new("https://example.com/bigbluebutton/", "secret");
    let mut request = CreateMeetingRequest::new();
    request.meeting_id = Some("12".to_string());
    request.moderator_pw = Some("modp".to_string());
    request.attandee_pw = Some("akarr".to_string());

    let _response = bbb.create_meeting(&request).await;
}
```

# API Implementation status

## Administration

- [x] create
- [ ] getDefaultConfigXML
- [ ] setConfigXML
- [x] end

## Monitoring

- [x] isMeetingRunning
- [x] getMeetings
- [x] getMeetingsInfo

## Recording

- [ ] getRecordings
- [x] publishRecordings
- [x] deleteRecordings
- [ ] updateRecordings
- [ ] getRecordingTextTracks
- [ ] putRecordingTextTrack

## Webhooks

- [x] Hooks/Create
- [x] Hooks/Destroy
- [x] Hooks/List

# bigbluebutton-rs 

[![Crates.io](https://img.shields.io/crates/v/bigbluebutton.svg)](https://crates.io/crates/bigbluebutton)
[![Documentation](https://docs.rs/bigbluebutton/badge.svg)](https://docs.rs/bigbluebutton/)
![build](https://github.com/KaranGauswami/bigbluebutton-rs/workflows/Rust/badge.svg)

Rust crate for interacting with BBB APIs.

BigBlueButton is an open source web conferencing system for online learning.

This crate provides an interface for interacting with Bigbluebutton APIs.
More details can be found [here](https://docs.bigbluebutton.org/dev/api.html).

# Examples

```rust
use bigbluebutton::Bigbluebutton;

// Creates new BBB Instance
let bbb = Bigbluebutton::new(
    "https://example.com/bigbluebutton/",
    "BBBSECRET",
  );

let params = vec![
    ("password", "pass"),
    ("fullName", "name"),
    ("meetingId", "1"),
  ];

let url = bbb.generate_url("join", params);

println!("{}",url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingId=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfdf
```

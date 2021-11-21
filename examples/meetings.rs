#![allow(unused_variables)]
// Create Meeting Example
use bigbluebutton::administration::CreateMeetingRequest;
use bigbluebutton::Bigbluebutton;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read API URL and Secret from Environement Variables
    let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");

    // create new client
    let client = Bigbluebutton::new(&bbb_url, &bbb_secret);

    // create new meeting request
    let mut request = CreateMeetingRequest::new("12");
    request.moderator_pw = Some("modp".to_string());
    request.attendee_pw = Some("akarr".to_string());
    request.record = Some(true);

    // execute new meeting request
    let response = client.create_meeting(&request).await?;

    eprintln!("{:?}", response);

    Ok(())
}

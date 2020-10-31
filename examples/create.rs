// Create Meeting Example
use bigbluebutton::administration::CreateMeetingRequest;
use bigbluebutton::{Bigbluebutton, Execute};

#[tokio::main]
async fn main() {
    let bbb = Bigbluebutton::new("https://example.com/bigbluebutton/", "secret");
    let mut request = CreateMeetingRequest::new();
    request.meeting_id = Some("12".to_string());
    request.moderator_pw = Some("modp".to_string());
    request.attandee_pw = Some("akarr".to_string());

    let _response = bbb.execute(&request).await;
}

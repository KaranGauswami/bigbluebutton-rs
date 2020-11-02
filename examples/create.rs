// Create Meeting Example
use bigbluebutton::administration::{CreateMeetingRequest, JoinMeetingRequest};
use bigbluebutton::{Bigbluebutton, Execute};

#[tokio::main]
async fn main() {
    let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
    let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);
    let mut request = CreateMeetingRequest::new("12");
    request.moderator_pw = Some("modp".to_string());
    request.attendee_pw = Some("akarr".to_string());

    let _response = bbb.execute(&request).await;

    let mut request = JoinMeetingRequest::new("Karan Gauswami", "12", "modp");

    let response = bbb.execute(&request).await;
    println!("{:?}", response);
}

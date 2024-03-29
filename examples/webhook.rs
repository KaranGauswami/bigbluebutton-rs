#![allow(unused_variables)]
// Create Meeting Example
use bigbluebutton::webhook::CreateHookRequest;
use bigbluebutton::Bigbluebutton;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
    let client = Bigbluebutton::new(&bbb_url, &bbb_secret);
    let mut request = CreateHookRequest::new("http://localhost:3003/callback");
    request.meeting_id = Some("24".to_string());
    let response = client.create_hook(&request).await?;

    // let mut request = ListHooksRequest::new();
    // let response = bbb.execute(&request).await?;

    Ok(())
}

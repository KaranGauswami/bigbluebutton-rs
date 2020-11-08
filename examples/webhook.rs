#![allow(unused_variables)]
// Create Meeting Example
use bigbluebutton::webhook::{CreateHookRequest, DestroyHookRequest};
use bigbluebutton::{Bigbluebutton, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
    let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);
    let mut request = CreateHookRequest::new("http%3A%2F%2Flocalhost%3A3003%2Fcallback");
    request.meeting_id = Some("24".to_string());
    let response = bbb.execute(&request).await?;

    println!("{:?}", response.return_code());
    println!("{:?}", response.hook_id());
    println!("{:?}", response.message_key());
    println!("{:?}", response.message());

    let mut request = DestroyHookRequest::new(response.hook_id());

    let response = bbb.execute(&request).await?;

    println!("{:?}", response.return_code());
    println!("{:?}", response.removed());
    Ok(())
}

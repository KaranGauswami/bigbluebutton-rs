#![allow(unused_variables)]
// Create Meeting Example
use bigbluebutton::recording::{DeleteRecordingsRequest, PublishRecordingsRequest};
use bigbluebutton::Bigbluebutton;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
    let client = Bigbluebutton::new(&bbb_url, &bbb_secret);

    let request = PublishRecordingsRequest::new("karan", false);

    let response = client.publish_recordings(&request).await?;
    println!("{:?}", response);

    let request = DeleteRecordingsRequest::new("123");

    let response = client.delete_recordings(&request).await?;
    println!("{:?}", response);

    Ok(())
}

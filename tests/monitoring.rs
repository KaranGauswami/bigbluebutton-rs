#[cfg(test)]
mod test {
    use bigbluebutton::administration::CreateMeetingRequest;
    use bigbluebutton::monitoring::{GetMeetingInfoRequest, GetMeetingsRequest};
    use bigbluebutton::{Bigbluebutton, Execute};

    #[test]
    #[ignore]
    fn get_meeting_info() {
        let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
        let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
        let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        rt.block_on(async {
            let mut request = CreateMeetingRequest::new("14");
            request.moderator_pw = Some("modp".to_string());
            request.attendee_pw = Some("attep".to_string());

            let _ = bbb.execute(&request).await;

            let mut request = GetMeetingInfoRequest::new();
            request.meeting_id = Some("14".to_string());

            let response = bbb.execute(&request).await.unwrap();
            assert_eq!(response.meeting_id(), &"14".to_string());
        });
    }

    #[test]
    #[ignore]
    fn get_meetings() {
        let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
        let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
        let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        rt.block_on(async {
            let mut request = CreateMeetingRequest::new("15");
            request.moderator_pw = Some("modp".to_string());
            request.attendee_pw = Some("attep".to_string());

            let mut request2 = CreateMeetingRequest::new("16");
            request2.moderator_pw = Some("modp".to_string());
            request2.attendee_pw = Some("attep".to_string());

            let _ = bbb.execute(&request).await;
            let _ = bbb.execute(&request2).await;

            let request = GetMeetingsRequest::new();

            let response = bbb.execute(&request).await.unwrap();
            assert_ne!(response.meetings().len(), 0);
        });
    }

    // #[test]
    // #[ignore]
    // fn is_meeting_running() {
    //     let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
    //     let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
    //     let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

    //     let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    //     rt.block_on(async {
    //         let mut request = CreateMeetingRequest::new("17");
    //         request.moderator_pw = Some("modp".to_string());
    //         request.attendee_pw = Some("attep".to_string());

    //         let _ = bbb.execute(&request).await;

    //         std::thread::sleep_ms(2000);

    //         let mut request = IsMeetingRunningRequest::new("17");

    //         let response = bbb.execute(&request).await.unwrap();
    //         assert_eq!(response.running, true);

    //         let request2 = EndMeetingRequest::new("17", "modp");
    //         let _ = bbb.execute(&request2).await;

    //         let response = bbb.execute(&request).await.unwrap();
    //         assert_eq!(response.running, false)
    //     });
    // }
}

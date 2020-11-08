#[cfg(test)]
mod test {
    use bigbluebutton::administration::{
        CreateMeetingRequest, EndMeetingRequest, JoinMeetingRequest,
    };
    use bigbluebutton::{Bigbluebutton, Execute};
    use std::env::var;

    #[test]
    #[ignore]
    fn create_meeting() {
        let bbb_url = var("BBB_URL").unwrap();
        let bbb_secret = var("BBB_SECRET").unwrap();
        let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        rt.block_on(async {
            let meeting_id = "1".to_string();
            let attendee_pw = "attendeep".to_string();
            let moderator_pw = "modp".to_string();
            let voice_bridge = "70757".to_string();
            let dial_number = "70757".to_string();
            let duration = 0;

            let mut request = CreateMeetingRequest::new(&meeting_id);

            request.attendee_pw = Some(attendee_pw.clone());
            request.moderator_pw = Some(moderator_pw.clone());
            request.voice_bridge = Some(voice_bridge.clone());
            request.dial_number = Some(dial_number.clone());
            request.duration = Some(duration.clone());

            let response = bbb.execute(&request).await.unwrap();
            assert_eq!(response.meeting_id(), &meeting_id);
            assert_eq!(response.attendee_pw(), &attendee_pw);
            assert_eq!(response.moderator_pw(), &moderator_pw);
            assert_eq!(response.voice_bridge(), &voice_bridge);
            assert_eq!(response.dial_number(), &dial_number);
            assert_eq!(response.duration(), &duration);
        });
    }

    #[test]
    #[ignore]
    fn end_meeting() {
        let bbb_url = var("BBB_URL").unwrap();
        let bbb_secret = var("BBB_SECRET").unwrap();
        let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        rt.block_on(async {
            let mut req = CreateMeetingRequest::new("2");
            req.moderator_pw = Some("modp".to_string());
            let _ = bbb.execute(&req).await;

            let req = EndMeetingRequest::new("2", "modp");

            let response = bbb.execute(&req).await.unwrap();
            println!("{:?}", response);
            assert_eq!(
                response.return_code,
                bigbluebutton::error::ResponseCode::SUCCESS
            );
            assert_eq!(response.message_key, "sentEndMeetingRequest".to_string());
        })
    }

    #[test]
    #[ignore]
    fn join_meeting() {
        let bbb_url = var("BBB_URL").unwrap();
        let bbb_secret = var("BBB_SECRET").unwrap();
        let bbb = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut rt = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        rt.block_on(async {
            let mut req = CreateMeetingRequest::new("3");
            req.moderator_pw = Some("modp".to_string());
            let _ = bbb.execute(&req).await;

            let req = JoinMeetingRequest::new("KaranGauswami", "3", "modp");

            let response = bbb.execute(&req).await.unwrap();
            assert_eq!(
                response.return_code,
                bigbluebutton::error::ResponseCode::SUCCESS
            );
            assert_eq!(response.message_key, "successfullyJoined".to_string());
        })
    }
}

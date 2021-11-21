#[cfg(test)]
mod test {
    use bigbluebutton::administration::CreateMeetingRequest;
    use bigbluebutton::monitoring::{GetMeetingInfoRequest, GetMeetingsRequest};
    use bigbluebutton::Bigbluebutton;

    #[tokio::test]
    #[ignore]
    async fn get_meeting_info() {
        let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
        let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
        let client = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut request = CreateMeetingRequest::new("14");
        request.moderator_pw = Some("modp".to_string());
        request.attendee_pw = Some("attep".to_string());

        let _ = client.create_meeting(&request).await;

        let mut request = GetMeetingInfoRequest::new();
        request.meeting_id = Some("14".to_string());

        let response = client
            .get_meeting_info(&request)
            .await
            .expect("Unable to parse GetMeetingInfoResponse");
        assert_eq!(response.meeting_id(), &"14".to_string());
    }

    #[tokio::test]
    #[ignore]
    async fn get_meetings() {
        let bbb_url = std::env::var("BBB_URL").expect("BBB_URL is not set");
        let bbb_secret = std::env::var("BBB_SECRET").expect("BBB_SECRET is not set");
        let client = Bigbluebutton::new(&bbb_url, &bbb_secret);

        let mut request = CreateMeetingRequest::new("15");
        request.moderator_pw = Some("modp".to_string());
        request.attendee_pw = Some("attep".to_string());

        let mut request2 = CreateMeetingRequest::new("16");
        request2.moderator_pw = Some("modp".to_string());
        request2.attendee_pw = Some("attep".to_string());

        let _ = client.create_meeting(&request).await;
        let _ = client.create_meeting(&request2).await;

        let request = GetMeetingsRequest::new();

        let response = client
            .get_meetings(&request)
            .await
            .expect("Unable to parse GetMeetingsResponse");
        assert_ne!(response.meetings().len(), 0);
    }
}

use crate::error::{BBBError, ResponseCode};
use crate::{helper, Bigbluebutton, Execute};
use async_trait::async_trait;
use bbb_macro::ApiName;
use helper::GetApiName;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call enables you to simply check on whether or not a meeting is running by looking it up with your meeting ID.
pub struct IsMeetingRunningRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    meeting_id: String,
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [IsMeetingRunningRequest]
pub struct IsMeetingRunningResponse {
    #[serde(rename = "returncode")]
    pub return_code: ResponseCode,

    pub running: Option<String>,
}
impl IsMeetingRunningRequest {
    /// Creates new IsMeetingRunningRequest
    pub fn new(meeting_id: &str) -> Self {
        Self {
            meeting_id: meeting_id.to_string(),
            api_name: "isMeetingRunning".to_string(),
        }
    }
}

#[async_trait]
impl Execute<IsMeetingRunningRequest, IsMeetingRunningResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &IsMeetingRunningRequest,
    ) -> Result<IsMeetingRunningResponse, BBBError> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call will return a list of all the meetings found on this server.
pub struct GetMeetingsRequest {
    api_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Attendee Details
pub struct Attendee {
    #[serde(rename = "userID")]
    pub user_id: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "role")]
    pub role: String,

    #[serde(rename = "isPresenter")]
    pub is_presenter: String,

    #[serde(rename = "isListeningOnly")]
    pub is_listening_only: bool,

    #[serde(rename = "hasJoinedVoice")]
    pub has_joined_voice: bool,

    #[serde(rename = "hasVideo")]
    pub has_video: bool,

    #[serde(rename = "clientType")]
    pub client_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Meeting details
pub struct Meeting {
    #[serde(rename = "meetingName")]
    pub meeting_name: String,

    #[serde(rename = "meetingID")]
    pub meeting_id: String,

    #[serde(rename = "internalMeetingID")]
    pub internal_meeting_id: String,

    #[serde(rename = "createTime")]
    pub create_time: String,

    #[serde(rename = "createDate")]
    pub create_date: String,

    #[serde(rename = "voiceBridge")]
    pub voice_bridge: String,

    #[serde(rename = "dialNumber")]
    pub dial_number: String,

    #[serde(rename = "attendeePW")]
    pub attendee_pw: String,

    #[serde(rename = "moderatorPW")]
    pub moderator_pw: String,

    #[serde(rename = "running")]
    pub running: String,

    #[serde(rename = "duration")]
    pub duration: String,

    #[serde(rename = "hasUserJoined")]
    pub has_user_joined: String,

    recording: String,

    #[serde(rename = "hasBeenForciblyEnded")]
    pub has_been_forcibly_ended: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    #[serde(rename = "participantCount")]
    pub participant_count: String,

    #[serde(rename = "listenerCount")]
    pub listener_count: String,

    #[serde(rename = "voiceParticipantCount")]
    pub voice_participant_count: String,

    #[serde(rename = "videoCount")]
    pub video_count: String,

    #[serde(rename = "maxUsers")]
    pub max_users: String,

    #[serde(rename = "moderatorCount")]
    pub moderator_count: String,

    #[serde(rename = "attendees")]
    #[serde(deserialize_with = "from_attendee")]
    pub attendees: Vec<Attendee>,

    #[serde(rename = "metadata")]
    pub metadata: String,

    #[serde(rename = "isBreakout")]
    pub is_breakout: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [GetMeetingsRequest]
pub struct GetMeetingsResponse {
    #[serde(rename = "returncode")]
    return_code: ResponseCode,
    #[serde(deserialize_with = "from_meeting")]
    meetings: Vec<Meeting>,
}
fn from_meeting<'de, D>(deserializer: D) -> Result<Vec<Meeting>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct MeetDetailsK {
        meeting: Vec<Meeting>,
    }

    let temp: MeetDetailsK = Deserialize::deserialize(deserializer)?;
    Ok(temp.meeting)
}
impl GetMeetingsRequest {
    /// Creates new GetMeetingsRequest
    pub fn new() -> Self {
        Self {
            api_name: "getMeetings".to_string(),
        }
    }
}

#[async_trait]
impl Execute<GetMeetingsRequest, GetMeetingsResponse> for Bigbluebutton {
    async fn execute(&self, request: &GetMeetingsRequest) -> Result<GetMeetingsResponse, BBBError> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call will return all of a meeting’s information, including the list of attendees as well as start and end times.
pub struct GetMeetingInfoRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    pub meeting_id: Option<String>,
    api_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Response return from [GetMeetingInfoRequest]
pub struct GetMeetingInfoResponse {
    #[serde(rename = "returncode")]
    return_code: String,

    #[serde(rename = "meetingName")]
    meeting_name: String,

    #[serde(rename = "meetingID")]
    meeting_id: String,

    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    #[serde(rename = "createTime")]
    create_time: String,

    #[serde(rename = "createDate")]
    create_date: String,

    #[serde(rename = "voiceBridge")]
    voice_bridge: String,

    #[serde(rename = "dialNumber")]
    dial_number: String,

    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    #[serde(rename = "running")]
    running: String,

    #[serde(rename = "duration")]
    duration: String,

    #[serde(rename = "hasUserJoined")]
    has_user_joined: String,

    recording: String,

    #[serde(rename = "hasBeenForciblyEnded")]
    has_been_forcibly_ended: String,

    #[serde(rename = "startTime")]
    start_time: String,

    #[serde(rename = "endTime")]
    end_time: String,

    #[serde(rename = "participantCount")]
    participant_count: String,

    #[serde(rename = "listenerCount")]
    listener_count: String,

    #[serde(rename = "voiceParticipantCount")]
    voice_participant_count: String,

    #[serde(rename = "videoCount")]
    video_count: String,

    #[serde(rename = "maxUsers")]
    max_users: String,

    #[serde(rename = "moderatorCount")]
    moderator_count: String,

    #[serde(rename = "attendees")]
    #[serde(deserialize_with = "from_attendee")]
    attendees: Vec<Attendee>,

    #[serde(rename = "metadata")]
    metadata: String,

    #[serde(rename = "isBreakout")]
    is_breakout: String,
}
fn from_attendee<'de, D>(deserializer: D) -> Result<Vec<Attendee>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct AttendeDetailsK {
        attendee: Vec<Attendee>,
    }

    let temp: AttendeDetailsK = Deserialize::deserialize(deserializer)?;
    Ok(temp.attendee)
}
impl GetMeetingInfoRequest {
    /// Creates new GetMeetingsRequest
    pub fn new() -> Self {
        Self {
            api_name: "getMeetingInfo".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<GetMeetingInfoRequest, GetMeetingInfoResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &GetMeetingInfoRequest,
    ) -> Result<GetMeetingInfoResponse, BBBError> {
        self.dispatch(request).await
    }
}

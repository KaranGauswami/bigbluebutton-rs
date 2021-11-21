use crate::error::ResponseCode;
use crate::{helper, Bigbluebutton, Execute};
use async_trait::async_trait;
use getset::Getters;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// This call enables you to simply check on whether or not a meeting is running by looking it up with your meeting ID.
pub struct IsMeetingRunningRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    meeting_id: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Response return from [IsMeetingRunningRequest]
pub struct IsMeetingRunningResponse {
    #[serde(rename = "returncode")]
    /// Return code
    pub return_code: ResponseCode,

    /// If meeting is running or not
    pub running: bool,
}
impl IsMeetingRunningRequest {
    /// Creates new IsMeetingRunningRequest
    pub fn new(meeting_id: impl ToString) -> Self {
        Self {
            meeting_id: meeting_id.to_string(),
        }
    }
}

#[async_trait]
impl Execute<IsMeetingRunningRequest, IsMeetingRunningResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &IsMeetingRunningRequest,
    ) -> anyhow::Result<IsMeetingRunningResponse> {
        self.dispatch("isMeetingRunning", request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// This call will return a list of all the meetings found on this server.
pub struct GetMeetingsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Attendee Details
pub struct Attendee {
    #[serde(rename = "userID")]
    /// User Id
    user_id: String,

    /// Full name of user
    full_name: String,

    /// User role
    role: String,

    /// If user is presenter
    is_presenter: String,

    /// If user is joined as a listen only mode
    is_listening_only: bool,

    /// If user is joined audio channel
    has_joined_voice: bool,

    /// If user is sharing video
    has_video: bool,

    /// Client Type
    client_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Meeting details
pub struct Meeting {
    /// Meeting name
    meeting_name: String,

    /// Meeting Id
    #[serde(rename = "meetingID")]
    meeting_id: String,

    /// Internal Meeting Id
    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    /// Create Time
    create_time: String,

    /// Create Date
    create_date: String,

    /// Voice Bridge
    voice_bridge: String,

    /// Dial Number
    dial_number: String,

    /// Attendee Password
    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    /// Moderator Password
    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    /// If meeting is running
    running: String,

    /// Meeting duration
    duration: String,

    /// Has user joined
    has_user_joined: String,

    /// Recording
    recording: String,

    /// Has been forcibly ended
    has_been_forcibly_ended: String,

    /// Start time
    start_time: String,

    /// End time
    end_time: String,

    /// Participant count
    participant_count: String,

    /// Listener count
    listener_count: String,

    /// Voice participant count
    voice_participant_count: String,

    /// Video count
    video_count: String,

    /// Max users
    max_users: String,

    /// Moderator count
    moderator_count: String,

    /// Attendees
    #[serde(deserialize_with = "from_attendee")]
    attendees: Vec<Attendee>,

    /// Metadata
    metadata: String,

    /// Is breakout
    is_breakout: String,
}

/// Response return from [GetMeetingsRequest]
#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct GetMeetingsResponse {
    /// Return code
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    /// Meetings
    #[serde(deserialize_with = "from_meeting")]
    meetings: Vec<Meeting>,
}
fn from_meeting<'de, D>(deserializer: D) -> Result<Vec<Meeting>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct MeetDetailsK {
        meeting: Option<Vec<Meeting>>,
    }

    let temp: MeetDetailsK = Deserialize::deserialize(deserializer)?;
    if let Some(value) = temp.meeting {
        Ok(value)
    } else {
        Ok(Vec::new())
    }
}
impl GetMeetingsRequest {
    /// Creates new GetMeetingsRequest
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Execute<GetMeetingsRequest, GetMeetingsResponse> for Bigbluebutton {
    async fn execute(&self, request: &GetMeetingsRequest) -> anyhow::Result<GetMeetingsResponse> {
        self.dispatch("getMeetings", request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// This call will return all of a meeting’s information, including the list of attendees as well as start and end times.
pub struct GetMeetingInfoRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    pub meeting_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Response return from [GetMeetingInfoRequest]
pub struct GetMeetingInfoResponse {
    /// Return code
    #[serde(rename = "returncode")]
    return_code: String,

    /// Meeting name
    meeting_name: String,

    /// Meeting Id
    #[serde(rename = "meetingID")]
    meeting_id: String,

    /// Internal Meeting Id
    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    /// Create Time
    create_time: String,

    /// Create Date
    create_date: String,

    /// Voice Bridge
    voice_bridge: String,

    /// Dial Number
    dial_number: String,

    /// Attendee Password
    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    /// Moderator Password
    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    /// If meeting is running
    running: String,

    /// Meeting duration
    duration: String,

    /// Has user joined
    has_user_joined: String,

    /// Recording
    recording: String,

    /// Has been forcibly ended
    has_been_forcibly_ended: String,

    /// Start time
    start_time: String,

    /// End time
    end_time: String,

    /// Participant count
    participant_count: String,

    /// Listener count
    listener_count: String,

    /// Voice participant count
    voice_participant_count: String,

    /// Video count
    video_count: String,

    /// Max users
    max_users: String,

    /// Moderator count
    moderator_count: String,

    /// Attendees
    #[serde(deserialize_with = "from_attendee")]
    attendees: Vec<Attendee>,

    /// Metadata
    metadata: String,

    /// Is breakout
    is_breakout: String,
}
impl GetMeetingInfoRequest {
    /// Creates new GetMeetingsRequest
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<GetMeetingInfoRequest, GetMeetingInfoResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &GetMeetingInfoRequest,
    ) -> anyhow::Result<GetMeetingInfoResponse> {
        self.dispatch("getMeetingInfo", request).await
    }
}

fn from_attendee<'de, D>(deserializer: D) -> Result<Vec<Attendee>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct AttendeDetailsK {
        attendee: Option<Vec<Attendee>>,
    }

    let temp: AttendeDetailsK = Deserialize::deserialize(deserializer)?;
    if let Some(value) = temp.attendee {
        Ok(value)
    } else {
        Ok(Vec::new())
    }
}

use crate::error::ResponseCode;
use crate::{helper, Bigbluebutton, Execute};
use async_trait::async_trait;
use bbb_macro::ApiName;
use getset::Getters;
use helper::GetApiName;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call enables you to simply check on whether or not a meeting is running by looking it up with your meeting ID.
pub struct IsMeetingRunningRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    meeting_id: String,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
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
            api_name: "isMeetingRunning".to_string(),
        }
    }
}

#[async_trait]
impl Execute<IsMeetingRunningRequest, IsMeetingRunningResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &IsMeetingRunningRequest,
    ) -> anyhow::Result<IsMeetingRunningResponse> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call will return a list of all the meetings found on this server.
pub struct GetMeetingsRequest {

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
/// Attendee Details
pub struct Attendee {
    #[serde(rename = "userID")]
    /// User Id
    user_id: String,

    #[serde(rename = "fullName")]
    /// Full name of user
    full_name: String,

    #[serde(rename = "role")]
    /// User role
    role: String,

    #[serde(rename = "isPresenter")]
    /// If user is presenter
    is_presenter: String,

    #[serde(rename = "isListeningOnly")]
    /// If user is joined as a listen only mode
    is_listening_only: bool,

    #[serde(rename = "hasJoinedVoice")]
    /// If user is joined audio channel
    has_joined_voice: bool,

    #[serde(rename = "hasVideo")]
    /// If user is sharing video
    has_video: bool,

    #[serde(rename = "clientType")]
    /// Client Type
    client_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Getters)]
#[getset(get = "pub")]
/// Meeting details
pub struct Meeting {
    /// Meeting name
    #[serde(rename = "meetingName")]
    meeting_name: String,

    /// Meeting Id
    #[serde(rename = "meetingID")]
    meeting_id: String,

    /// Internal Meeting Id
    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    /// Create Time
    #[serde(rename = "createTime")]
    create_time: String,

    /// Create Date
    #[serde(rename = "createDate")]
    create_date: String,

    /// Voice Bridge
    #[serde(rename = "voiceBridge")]
    voice_bridge: String,

    /// Dial Number
    #[serde(rename = "dialNumber")]
    dial_number: String,

    /// Attendee Password
    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    /// Moderator Password
    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    /// If meeting is running
    #[serde(rename = "running")]
    running: String,

    /// Meeting duration
    #[serde(rename = "duration")]
    duration: String,

    /// Has user joined
    #[serde(rename = "hasUserJoined")]
    has_user_joined: String,

    /// Recording
    recording: String,

    /// Has been forcibly ended
    #[serde(rename = "hasBeenForciblyEnded")]
    has_been_forcibly_ended: String,

    /// Start time
    #[serde(rename = "startTime")]
    start_time: String,

    /// End time
    #[serde(rename = "endTime")]
    end_time: String,

    /// Participant count
    #[serde(rename = "participantCount")]
    participant_count: String,

    /// Listener count
    #[serde(rename = "listenerCount")]
    listener_count: String,

    /// Voice participant count
    #[serde(rename = "voiceParticipantCount")]
    voice_participant_count: String,

    /// Video count
    #[serde(rename = "videoCount")]
    video_count: String,

    /// Max users
    #[serde(rename = "maxUsers")]
    max_users: String,

    /// Moderator count
    #[serde(rename = "moderatorCount")]
    moderator_count: String,

    /// Attendees
    #[serde(rename = "attendees")]
    #[serde(deserialize_with = "from_attendee")]
    attendees: Vec<Attendee>,

    /// Metadata
    #[serde(rename = "metadata")]
    metadata: String,

    /// Is breakout
    #[serde(rename = "isBreakout")]
    is_breakout: String,
}

/// Response return from [GetMeetingsRequest]
#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
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
        Self {
            api_name: "getMeetings".to_string(),
        }
    }
}

#[async_trait]
impl Execute<GetMeetingsRequest, GetMeetingsResponse> for Bigbluebutton {
    async fn execute(&self, request: &GetMeetingsRequest) -> anyhow::Result<GetMeetingsResponse> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// This call will return all of a meeting’s information, including the list of attendees as well as start and end times.
pub struct GetMeetingInfoRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    pub meeting_id: Option<String>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Getters)]
#[getset(get = "pub")]
/// Response return from [GetMeetingInfoRequest]
pub struct GetMeetingInfoResponse {
    /// Return code
    #[serde(rename = "returncode")]
    return_code: String,

    /// Meeting name
    #[serde(rename = "meetingName")]
    meeting_name: String,

    /// Meeting Id
    #[serde(rename = "meetingID")]
    meeting_id: String,

    /// Internal Meeting Id
    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    /// Create Time
    #[serde(rename = "createTime")]
    create_time: String,

    /// Create Date
    #[serde(rename = "createDate")]
    create_date: String,

    /// Voice Bridge
    #[serde(rename = "voiceBridge")]
    voice_bridge: String,

    /// Dial Number
    #[serde(rename = "dialNumber")]
    dial_number: String,

    /// Attendee Password
    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    /// Moderator Password
    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    /// If meeting is running
    #[serde(rename = "running")]
    running: String,

    /// Meeting duration
    #[serde(rename = "duration")]
    duration: String,

    /// Has user joined
    #[serde(rename = "hasUserJoined")]
    has_user_joined: String,

    /// Recording
    recording: String,

    /// Has been forcibly ended
    #[serde(rename = "hasBeenForciblyEnded")]
    has_been_forcibly_ended: String,

    /// Start time
    #[serde(rename = "startTime")]
    start_time: String,

    /// End time
    #[serde(rename = "endTime")]
    end_time: String,

    /// Participant count
    #[serde(rename = "participantCount")]
    participant_count: String,

    /// Listener count
    #[serde(rename = "listenerCount")]
    listener_count: String,

    /// Voice participant count
    #[serde(rename = "voiceParticipantCount")]
    voice_participant_count: String,

    /// Video count
    #[serde(rename = "videoCount")]
    video_count: String,

    /// Max users
    #[serde(rename = "maxUsers")]
    max_users: String,

    /// Moderator count
    #[serde(rename = "moderatorCount")]
    moderator_count: String,

    /// Attendees
    #[serde(rename = "attendees")]
    #[serde(deserialize_with = "from_attendee")]
    attendees: Vec<Attendee>,

    /// Metadata
    #[serde(rename = "metadata")]
    metadata: String,

    /// Is breakout
    #[serde(rename = "isBreakout")]
    is_breakout: String,
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
    ) -> anyhow::Result<GetMeetingInfoResponse> {
        self.dispatch(request).await
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

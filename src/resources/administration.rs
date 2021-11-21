use crate::error::ResponseCode;
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use bbb_macro::ApiName;
use getset::Getters;
use helper::GetApiName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
#[serde(rename_all = "camelCase")]
/// Creates a BigBlueButton meeting.
pub struct CreateMeetingRequest {
    /// A name for the meeting.
    pub name: Option<String>,

    #[serde(rename = "meetingID")]
    /// A meeting ID that can be used to identify this meeting by the 3rd-party application.
    pub meeting_id: String,

    #[serde(rename = "attendeePW")]
    /// The password that the join URL can later provide as its password parameter to indicate the user will join as a viewer. If no attendeePW is provided, the create call will return a randomly generated attendeePW password for the meeting.
    pub attendee_pw: Option<String>,

    #[serde(rename = "moderatorPW")]
    /// The password that will join URL can later provide as its password parameter to indicate the user will as a moderator. if no moderatorPW is provided, create will return a randomly generated moderatorPW password for the meeting.
    pub moderator_pw: Option<String>,

    /// A welcome message that gets displayed on the chat window when the participant joins.
    pub welcome: Option<String>,

    /// The dial access number that participants can call in using regular phone.
    pub dial_number: Option<String>,

    /// Voice conference number for the FreeSWITCH voice conference associated with this meeting.
    pub voice_bridge: Option<String>,

    /// Set the maximum number of users allowed to joined the conference at the same time.
    pub max_participants: Option<u64>,

    #[serde(rename = "logoutURL")]
    /// The URL that the BigBlueButton client will go to after users click the OK button on the ‘You have been logged out message’.
    pub logout_url: Option<String>,

    /// Setting ‘record=true’ instructs the BigBlueButton server to record the media and events in the session for later playback. The default is false.
    pub record: Option<bool>,

    /// The maximum length (in minutes) for the meeting.
    pub duration: Option<u64>,

    /// Must be set to true to create a breakout room.
    pub is_breakout: Option<bool>,

    #[serde(rename = "parentMeetingID")]
    /// Must be provided when creating a breakout room, the parent room must be running.
    pub parent_meeting_id: Option<String>,

    /// The sequence number of the breakout room.
    pub sequence: Option<u64>,

    /// If set to true, the client will give the user the choice to choose the breakout rooms he wants to join.
    pub free_join: Option<bool>,

    /// You can pass one or more metadata values when creating a meeting. These will be stored by BigBlueButton can be retrieved later via the getMeetingInfo and getRecordings calls.
    pub meta: Option<String>,

    /// Display a message to all moderators in the public chat.
    pub moderator_only_message: Option<String>,

    /// Whether to automatically start recording when first user joins (default false).
    pub auto_start_recording: Option<bool>,

    /// Allow the user to start/stop recording. (default true)
    pub allow_start_stop_recording: Option<bool>,

    /// Setting webcamsOnlyForModerator=true will cause all webcams shared by viewers during this meeting to only appear for moderators
    pub webcams_only_for_moderator: Option<bool>,

    /// Setting logo=<http://www.example.com/my-custom-logo.png> will replace the default logo in the Flash client.
    pub logo: Option<String>,

    /// Will set the banner text in the client.
    pub banner_text: Option<String>,

    /// Will set the banner background color in the client. The required format is color hex #FFFFFF.
    pub banner_color: Option<String>,

    /// Setting copyright=My custom copyright will replace the default copyright on the footer of the Flash client.
    pub copyright: Option<String>,

    /// Setting muteOnStart=true will mute all users when the meeting starts.
    pub mute_on_start: Option<bool>,

    /// Default allowModsToUnmuteUsers=false. Setting to allowModsToUnmuteUsers=true will allow moderators to unmute other users in the meeting.
    pub allow_mods_to_unmute_users: Option<bool>,

    /// Default lockSettingsDisableCam=false. Setting lockSettingsDisableCam=true will prevent users from sharing their camera in the meeting.
    pub lock_settings_disable_cam: Option<bool>,

    /// Default lockSettingsDisableMic=false. Setting to lockSettingsDisableMic=true will only allow user to join listen only.
    pub lock_settings_disable_mic: Option<bool>,

    /// Default lockSettingsDisablePrivateChat=false. Setting to lockSettingsDisablePrivateChat=true will disable private chats in the meeting.
    pub lock_settings_disable_private_chat: Option<bool>,

    /// Default lockSettingsDisablePublicChat=false. Setting to lockSettingsDisablePublicChat=true will disable public chat in the meeting
    pub lock_settings_disable_public_chat: Option<bool>,

    /// Default lockSettingsDisableNote=false. Setting to lockSettingsDisableNote=true will disable notes in the meeting.
    pub lock_settings_disable_note: Option<bool>,

    /// Default lockSettingsLockedLayout=false. Setting to lockSettingsLockedLayout=true will lock the layout in the meeting.
    pub lock_settings_locked_layout: Option<bool>,

    /// Default lockSettingsLockOnJoin=true. Setting to lockSettingsLockOnJoin=false will not apply lock setting to users when they join.
    pub lock_settings_lock_on_join: Option<bool>,

    /// Default lockSettingsLockOnJoinConfigurable=false. Setting to lockSettingsLockOnJoinConfigurable=true will allow applying of lockSettingsLockOnJoin param.
    pub lock_settings_lock_on_join_configurable: Option<bool>,

    /// Default guestPolicy=ALWAYS_ACCEPT. Will set the guest policy for the meeting. The guest policy determines whether or not users who send a join request with guest=true will be allowed to join the meeting. Possible values are ALWAYS_ACCEPT, ALWAYS_DENY, and ASK_MODERATOR.
    pub guest_policy: Option<String>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Response return from [CreateMeetingRequest]
pub struct CreateMeetingResponse {
    /// return code of meeting
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    /// A meeting ID that can be used to identify this meeting by the 3rd-party application.
    #[serde(rename = "meetingID")]
    meeting_id: String,

    /// Internal Meeting Id
    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: String,

    /// Parent Meeting Id
    #[serde(rename = "parentMeetingID")]
    parent_meeting_id: String,

    /// The password that the join URL can later provide as its password parameter to indicate the user will join as a viewer. If no attendeePW is provided, the create call will return a randomly generated attendeePW password for the meeting.
    #[serde(rename = "attendeePW")]
    attendee_pw: String,

    /// The password that will join URL can later provide as its password parameter to indicate the user will as a moderator. if no moderatorPW is provided, create will return a randomly generated moderatorPW password for the meeting.
    #[serde(rename = "moderatorPW")]
    moderator_pw: String,

    /// Meeting Created time
    create_time: u64,

    /// Voice conference number for the FreeSWITCH voice conference associated with this meeting.
    voice_bridge: String,

    /// The dial access number that participants can call in using regular phone.
    dial_number: String,

    /// Create Date
    create_date: String,

    /// If user is joined or not
    has_user_joined: bool,

    /// The maximum length (in minutes) for the meeting.
    duration: u64,

    /// Has been forcibly ended
    has_been_forcibly_ended: bool,

    /// Message Key
    message_key: String,

    /// Message
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
#[serde(rename_all = "camelCase")]
/// Use this to forcibly end a meeting and kick all participants out of the meeting.
pub struct EndMeetingRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to end.
    pub meeting_id: String,
    /// The moderator password for this meeting. You can not end a meeting using the attendee password.
    pub password: String,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
/// Response return from [EndMeetingRequest]
pub struct EndMeetingResponse {
    #[serde(rename = "returncode")]
    /// Return code
    pub return_code: ResponseCode,

    /// Message Key
    pub message_key: String,

    /// Message
    pub message: String,
}

impl CreateMeetingRequest {
    /// Creates new CreateMeetingRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::administration::CreateMeetingRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = CreateMeetingRequest::new("12");
    /// bbb.execute(&request);
    /// ```
    pub fn new(meeting_id: impl ToString) -> Self {
        Self {
            meeting_id: meeting_id.to_string(),
            api_name: "create".to_string(),
            ..Default::default()
        }
    }
}

impl EndMeetingRequest {
    /// creates new EndMeetingRequest
    pub fn new(meeting_id: impl ToString, password: impl ToString) -> Self {
        Self {
            meeting_id: meeting_id.to_string(),
            password: password.to_string(),
            api_name: "end".to_string(),
        }
    }
}

#[async_trait]
impl Execute<CreateMeetingRequest, CreateMeetingResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &CreateMeetingRequest,
    ) -> anyhow::Result<CreateMeetingResponse> {
        self.dispatch(request).await
    }
}

#[async_trait]
impl Execute<EndMeetingRequest, EndMeetingResponse> for Bigbluebutton {
    async fn execute(&self, request: &EndMeetingRequest) -> anyhow::Result<EndMeetingResponse> {
        self.dispatch(request).await
    }
}

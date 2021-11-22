use crate::{error::ResponseCode, Bigbluebutton};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, Setters, MutGetters, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
/// Creates a BigBlueButton meeting.
pub struct CreateMeetingRequest {
    /// A name for the meeting.
    name: Option<String>,

    #[serde(rename = "meetingID")]
    /// A meeting ID that can be used to identify this meeting by the 3rd-party application.
    meeting_id: String,

    #[serde(rename = "attendeePW")]
    /// The password that the join URL can later provide as its password parameter to indicate the user will join as a viewer. If no attendeePW is provided, the create call will return a randomly generated attendeePW password for the meeting.
    attendee_pw: Option<String>,

    #[serde(rename = "moderatorPW")]
    /// The password that will join URL can later provide as its password parameter to indicate the user will as a moderator. if no moderatorPW is provided, create will return a randomly generated moderatorPW password for the meeting.
    moderator_pw: Option<String>,

    /// A welcome message that gets displayed on the chat window when the participant joins.
    welcome: Option<String>,

    /// The dial access number that participants can call in using regular phone.
    dial_number: Option<String>,

    /// Voice conference number for the FreeSWITCH voice conference associated with this meeting.
    voice_bridge: Option<String>,

    /// Set the maximum number of users allowed to joined the conference at the same time.
    max_participants: Option<u64>,

    #[serde(rename = "logoutURL")]
    /// The URL that the BigBlueButton client will go to after users click the OK button on the ‘You have been logged out message’.
    logout_url: Option<String>,

    /// Setting ‘record=true’ instructs the BigBlueButton server to record the media and events in the session for later playback. The default is false.
    record: Option<bool>,

    /// The maximum length (in minutes) for the meeting.
    duration: Option<u64>,

    /// Must be set to true to create a breakout room.
    is_breakout: Option<bool>,

    #[serde(rename = "parentMeetingID")]
    /// Must be provided when creating a breakout room, the parent room must be running.
    parent_meeting_id: Option<String>,

    /// The sequence number of the breakout room.
    sequence: Option<u64>,

    /// If set to true, the client will give the user the choice to choose the breakout rooms he wants to join.
    free_join: Option<bool>,

    /// You can pass one or more metadata values when creating a meeting. These will be stored by BigBlueButton can be retrieved later via the getMeetingInfo and getRecordings calls.
    meta: Option<String>,

    /// Display a message to all moderators in the public chat.
    moderator_only_message: Option<String>,

    /// Whether to automatically start recording when first user joins (default false).
    auto_start_recording: Option<bool>,

    /// Allow the user to start/stop recording. (default true)
    allow_start_stop_recording: Option<bool>,

    /// Setting webcamsOnlyForModerator=true will cause all webcams shared by viewers during this meeting to only appear for moderators
    webcams_only_for_moderator: Option<bool>,

    /// Setting logo=<http://www.example.com/my-custom-logo.png> will replace the default logo in the Flash client.
    logo: Option<String>,

    /// Will set the banner text in the client.
    banner_text: Option<String>,

    /// Will set the banner background color in the client. The required format is color hex #FFFFFF.
    banner_color: Option<String>,

    /// Setting copyright=My custom copyright will replace the default copyright on the footer of the Flash client.
    copyright: Option<String>,

    /// Setting muteOnStart=true will mute all users when the meeting starts.
    mute_on_start: Option<bool>,

    /// Default allowModsToUnmuteUsers=false. Setting to allowModsToUnmuteUsers=true will allow moderators to unmute other users in the meeting.
    allow_mods_to_unmute_users: Option<bool>,

    /// Default lockSettingsDisableCam=false. Setting lockSettingsDisableCam=true will prevent users from sharing their camera in the meeting.
    lock_settings_disable_cam: Option<bool>,

    /// Default lockSettingsDisableMic=false. Setting to lockSettingsDisableMic=true will only allow user to join listen only.
    lock_settings_disable_mic: Option<bool>,

    /// Default lockSettingsDisablePrivateChat=false. Setting to lockSettingsDisablePrivateChat=true will disable private chats in the meeting.
    lock_settings_disable_private_chat: Option<bool>,

    /// Default lockSettingsDisablePublicChat=false. Setting to lockSettingsDisablePublicChat=true will disable public chat in the meeting
    lock_settings_disable_public_chat: Option<bool>,

    /// Default lockSettingsDisableNote=false. Setting to lockSettingsDisableNote=true will disable notes in the meeting.
    lock_settings_disable_note: Option<bool>,

    /// Default lockSettingsLockedLayout=false. Setting to lockSettingsLockedLayout=true will lock the layout in the meeting.
    lock_settings_locked_layout: Option<bool>,

    /// Default lockSettingsLockOnJoin=true. Setting to lockSettingsLockOnJoin=false will not apply lock setting to users when they join.
    lock_settings_lock_on_join: Option<bool>,

    /// Default lockSettingsLockOnJoinConfigurable=false. Setting to lockSettingsLockOnJoinConfigurable=true will allow applying of lockSettingsLockOnJoin param.
    lock_settings_lock_on_join_configurable: Option<bool>,

    /// Default guestPolicy=ALWAYS_ACCEPT. Will set the guest policy for the meeting. The guest policy determines whether or not users who send a join request with guest=true will be allowed to join the meeting. Possible values are ALWAYS_ACCEPT, ALWAYS_DENY, and ASK_MODERATOR.
    guest_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
/// Use this to forcibly end a meeting and kick all participants out of the meeting.
pub struct EndMeetingRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to end.
    meeting_id: String,
    /// The moderator password for this meeting. You can not end a meeting using the attendee password.
    password: String,
}

#[derive(Debug, Clone, Deserialize, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
/// Response return from [EndMeetingRequest]
pub struct EndMeetingResponse {
    #[serde(rename = "returncode")]
    /// Return code
    return_code: ResponseCode,

    /// Message Key
    message_key: String,

    /// Message
    message: String,
}

impl CreateMeetingRequest {
    /// Creates new CreateMeetingRequest
    ///
    /// ```rust
    /// # use bigbluebutton::Bigbluebutton;
    /// use bigbluebutton::administration::CreateMeetingRequest;
    /// let client = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = CreateMeetingRequest::new("12");
    /// client.create_meeting(&request);
    /// ```
    pub fn new(meeting_id: impl ToString) -> Self {
        Self {
            meeting_id: meeting_id.to_string(),
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
        }
    }
}

impl Bigbluebutton {
    pub async fn create_meeting(
        &self,
        req: &CreateMeetingRequest,
    ) -> Result<CreateMeetingResponse, anyhow::Error> {
        self.dispatch("create", req).await
    }
    pub async fn end_meeting(
        &self,
        req: &EndMeetingRequest,
    ) -> Result<EndMeetingResponse, anyhow::Error> {
        self.dispatch("end", req).await
    }
}

use crate::error::{BBBError, ErrorCode};
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
/// Creates a BigBlueButton meeting.

pub struct CreateMeetingRequest {
    #[serde(rename = "name")]
    /// A name for the meeting.
    pub name: Option<String>,

    #[serde(rename = "meetingID")]
    /// A meeting ID that can be used to identify this meeting by the 3rd-party application.
    pub meeting_id: Option<String>,

    #[serde(rename = "attendeePW")]
    /// The password that the join URL can later provide as its password parameter to indicate the user will join as a viewer. If no attendeePW is provided, the create call will return a randomly generated attendeePW password for the meeting.
    pub attandee_pw: Option<String>,

    #[serde(rename = "moderatorPW")]
    /// The password that will join URL can later provide as its password parameter to indicate the user will as a moderator. if no moderatorPW is provided, create will return a randomly generated moderatorPW password for the meeting.
    pub moderator_pw: Option<String>,

    #[serde(rename = "welcome")]
    /// A welcome message that gets displayed on the chat window when the participant joins.
    pub welcome: Option<String>,

    #[serde(rename = "dialNumber")]
    /// The dial access number that participants can call in using regular phone.
    pub dial_number: Option<String>,

    #[serde(rename = "voiceBridge")]
    /// Voice conference number for the FreeSWITCH voice conference associated with this meeting.
    pub voice_bridge: Option<String>,

    #[serde(rename = "maxParticipants")]
    /// Set the maximum number of users allowed to joined the conference at the same time.
    pub max_participants: Option<u64>,

    #[serde(rename = "logoutURL")]
    /// The URL that the BigBlueButton client will go to after users click the OK button on the ‘You have been logged out message’.
    pub logout_url: Option<String>,

    /// Setting ‘record=true’ instructs the BigBlueButton server to record the media and events in the session for later playback. The default is false.
    pub record: Option<bool>,

    /// The maximum length (in minutes) for the meeting.
    pub duration: Option<u64>,

    #[serde(rename = "isBreakout")]
    /// Must be set to true to create a breakout room.
    pub is_breakout: Option<bool>,

    #[serde(rename = "parentMeetingID")]
    /// Must be provided when creating a breakout room, the parent room must be running.
    pub parent_meeting_id: Option<String>,

    /// The sequence number of the breakout room.
    pub sequence: Option<u64>,

    #[serde(rename = "freeJoin")]
    /// If set to true, the client will give the user the choice to choose the breakout rooms he wants to join.
    pub free_join: Option<bool>,

    /// You can pass one or more metadata values when creating a meeting. These will be stored by BigBlueButton can be retrieved later via the getMeetingInfo and getRecordings calls.
    pub meta: Option<String>,

    #[serde(rename = "moderatorOnlyMessage")]
    /// Display a message to all moderators in the public chat.
    pub moderator_only_message: Option<String>,

    #[serde(rename = "autoStartRecording")]
    /// Whether to automatically start recording when first user joins (default false).
    pub auto_start_recording: Option<bool>,

    #[serde(rename = "allowStartStopRecording")]
    /// Allow the user to start/stop recording. (default true)
    pub allow_start_stop_recording: Option<bool>,

    #[serde(rename = "webcamsOnlyForModerator")]
    /// Setting webcamsOnlyForModerator=true will cause all webcams shared by viewers during this meeting to only appear for moderators
    pub webcams_only_for_moderator: Option<bool>,

    /// Setting logo=http://www.example.com/my-custom-logo.png will replace the default logo in the Flash client.
    pub logo: Option<String>,

    #[serde(rename = "bannerText")]
    /// Will set the banner text in the client.
    pub banner_text: Option<String>,

    #[serde(rename = "bannerColor")]
    /// Will set the banner background color in the client. The required format is color hex #FFFFFF.
    pub banner_color: Option<String>,

    /// Setting copyright=My custom copyright will replace the default copyright on the footer of the Flash client.
    pub copyright: Option<String>,

    /// Setting muteOnStart=true will mute all users when the meeting starts.
    #[serde(rename = "muteOnStart")]
    pub mute_on_start: Option<bool>,

    #[serde(rename = "allowModsToUnmuteUsers")]
    /// Default allowModsToUnmuteUsers=false. Setting to allowModsToUnmuteUsers=true will allow moderators to unmute other users in the meeting.
    pub allow_mods_to_unmute_users: Option<bool>,

    #[serde(rename = "lockSettingsDisableCam")]
    /// Default lockSettingsDisableCam=false. Setting lockSettingsDisableCam=true will prevent users from sharing their camera in the meeting.
    pub lock_settings_disable_cam: Option<bool>,

    #[serde(rename = "lockSettingsDisableMic")]
    /// Default lockSettingsDisableMic=false. Setting to lockSettingsDisableMic=true will only allow user to join listen only.
    pub lock_settings_disable_mic: Option<bool>,

    #[serde(rename = "lockSettingsDisablePrivateChat")]
    /// Default lockSettingsDisablePrivateChat=false. Setting to lockSettingsDisablePrivateChat=true will disable private chats in the meeting.
    pub lock_settings_disable_private_chat: Option<bool>,

    #[serde(rename = "lockSettingsDisablePublicChat")]
    /// Default lockSettingsDisablePublicChat=false. Setting to lockSettingsDisablePublicChat=true will disable public chat in the meeting
    pub lock_settings_disable_public_chat: Option<bool>,

    #[serde(rename = "lockSettingsDisableNote")]
    /// Default lockSettingsDisableNote=false. Setting to lockSettingsDisableNote=true will disable notes in the meeting.
    pub lock_settings_disable_note: Option<bool>,

    #[serde(rename = "lockSettingsLockedLayout")]
    /// Default lockSettingsLockedLayout=false. Setting to lockSettingsLockedLayout=true will lock the layout in the meeting.
    pub lock_settings_locked_layout: Option<bool>,

    #[serde(rename = "lockSettingsLockOnJoin")]
    /// Default lockSettingsLockOnJoin=true. Setting to lockSettingsLockOnJoin=false will not apply lock setting to users when they join.
    pub lock_settings_lock_on_join: Option<bool>,

    #[serde(rename = "lockSettingsLockOnJoinConfigurable")]
    /// Default lockSettingsLockOnJoinConfigurable=false. Setting to lockSettingsLockOnJoinConfigurable=true will allow applying of lockSettingsLockOnJoin param.
    pub lock_settings_lock_on_join_configurable: Option<bool>,

    #[serde(rename = "guestPolicy")]
    /// Default guestPolicy=ALWAYS_ACCEPT. Will set the guest policy for the meeting. The guest policy determines whether or not users who send a join request with guest=true will be allowed to join the meeting. Possible values are ALWAYS_ACCEPT, ALWAYS_DENY, and ASK_MODERATOR.
    pub guest_policy: Option<String>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [CreateMeetingRequest]
pub struct CreateMeetingResponse {
    #[serde(rename = "returncode")]
    return_code: ErrorCode,

    #[serde(rename = "meetingID")]
    meeting_id: Option<String>,

    #[serde(rename = "internalMeetingID")]
    internal_meeting_id: Option<String>,

    #[serde(rename = "parentMeetingID")]
    parent_meeting_id: Option<String>,

    #[serde(rename = "attendeePW")]
    attendee_pw: Option<String>,

    #[serde(rename = "moderatorPW")]
    moderator_pw: Option<String>,

    #[serde(rename = "createTime")]
    create_time: Option<String>,

    #[serde(rename = "voiceBridge")]
    voice_bridge: Option<String>,

    #[serde(rename = "dialNumber")]
    dial_number: Option<String>,

    #[serde(rename = "createDate")]
    create_date: Option<String>,

    #[serde(rename = "hasUserJoined")]
    has_user_joined: Option<String>,

    #[serde(rename = "duration")]
    duration: Option<String>,

    #[serde(rename = "hasBeenForciblyEnded")]
    has_been_forcibly_ended: Option<String>,

    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    #[serde(rename = "message")]
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// Joins a user to the meeting specified in the meetingID parameter.
pub struct JoinMeetingRequest {
    #[serde(rename = "fullName")]
    /// The full name that is to be used to identify this user to other conference attendees.
    pub full_name: Option<String>,

    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to join.
    pub meeting_id: Option<String>,

    #[serde(rename = "password")]
    /// The password that this attendee is using. If the moderator password is supplied, he will be given moderator status (and the same for attendee password, etc)
    pub password: Option<String>,

    #[serde(rename = "createTime")]
    /// Third-party apps using the API can now pass createTime parameter (which was created in the create call), BigBlueButton will ensure it matches the ‘createTime’ for the session. If they differ, BigBlueButton will not proceed with the join request. This prevents a user from reusing their join URL for a subsequent session with the same meetingID.
    pub create_time: Option<String>,

    #[serde(rename = "userID")]
    /// An identifier for this user that will help your application to identify which person this is. This user ID will be returned for this user in the getMeetingInfo API call so that you can check
    pub user_id: Option<String>,

    #[serde(rename = "webVoiceConf")]
    /// If you want to pass in a custom voice-extension when a user joins the voice conference using voip. This is useful if you want to collect more info in you Call Detail Records about the user joining the conference. You need to modify your /etc/asterisk/bbb-extensions.conf to handle this new extensions.
    pub web_voice_conf: Option<String>,

    #[serde(rename = "configToken")]
    /// The token returned by a setConfigXML API call. This causes the BigBlueButton client to load the config.xml associated with the token (not the default config.xml)
    pub config_token: Option<String>,

    #[serde(rename = "defaultLayout")]
    /// The layout name to be loaded first when the application is loaded.
    pub default_layout: Option<u64>,

    #[serde(rename = "avatarURL")]
    /// The link for the user’s avatar to be displayed when displayAvatar in config.xml is set to true (not yet implemented in the HTML5 client, see [#8566](https://github.com/bigbluebutton/bigbluebutton/issues/8566).
    pub avatar_url: Option<String>,

    redirect: bool,

    #[serde(rename = "clientURL")]
    /// Some third party apps what to display their own custom client. These apps can pass the URL containing the custom client and when redirect is not set to false, the browser will get redirected to the value of clientURL.
    pub client_url: Option<bool>,

    #[serde(rename = "joinViaHtml5")]
    /// Set to “true” to force the HTML5 client to load for the user.
    pub join_via_html5: Option<u64>,

    /// Set to “true” to indicate that the user is a guest, otherwise do NOT send this parameter.
    pub guest: Option<bool>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [JoinMeetingRequest]
pub struct JoinMeetingResponse {
    #[serde(rename = "returncode")]
    return_code: ErrorCode,

    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    message: Option<String>,

    meeting_id: Option<String>,

    user_id: Option<String>,

    auth_token: Option<String>,

    session_token: Option<String>,

    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// Use this to forcibly end a meeting and kick all participants out of the meeting.
pub struct EndMeetingRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to end.
    pub meeting_id: Option<String>,
    /// The moderator password for this meeting. You can not end a meeting using the attendee password.
    pub password: Option<String>,
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [EndMeetingRequest]
pub struct EndMeetingResponse {
    #[serde(rename = "returncode")]
    return_code: ErrorCode,

    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    #[serde(rename = "message")]
    message: Option<String>,
}

impl CreateMeetingRequest {
    /// Creates new CreateMeetingRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::administration::CreateMeetingRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = CreateMeetingRequest::new();
    /// request.meeting_id = Some("12".to_string());
    /// bbb.execute(&request);
    /// ```
    pub fn new() -> Self {
        Self {
            api_name: "create".to_string(),
            ..Default::default()
        }
    }
}
impl JoinMeetingRequest {
    /// creates new JoinMeetingRequest
    pub fn new() -> Self {
        Self {
            api_name: "join".to_string(),
            redirect: false,
            ..Default::default()
        }
    }
}
impl EndMeetingRequest {
    /// creates new EndMeetingRequest
    pub fn new() -> Self {
        Self {
            api_name: "end".to_string(),
            ..Default::default()
        }
    }
}

impl helper::GetApiName for CreateMeetingRequest {
    fn get_api_name(&self) -> &str {
        &self.api_name
    }
}
impl helper::GetApiName for JoinMeetingRequest {
    fn get_api_name(&self) -> &str {
        &self.api_name
    }
}
impl helper::GetApiName for EndMeetingRequest {
    fn get_api_name(&self) -> &str {
        &self.api_name
    }
}

#[async_trait]
impl Execute<CreateMeetingRequest, CreateMeetingResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &CreateMeetingRequest,
    ) -> Result<CreateMeetingResponse, BBBError> {
        self.dispatch(request).await
    }
}

#[async_trait]
impl Execute<JoinMeetingRequest, JoinMeetingResponse> for Bigbluebutton {
    async fn execute(&self, request: &JoinMeetingRequest) -> Result<JoinMeetingResponse, BBBError> {
        self.dispatch(request).await
    }
}

#[async_trait]
impl Execute<EndMeetingRequest, EndMeetingResponse> for Bigbluebutton {
    async fn execute(&self, request: &EndMeetingRequest) -> Result<EndMeetingResponse, BBBError> {
        self.dispatch(request).await
    }
}

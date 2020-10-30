use super::error::{BBBError, ErrorCode};
use super::helper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateMeetingRequest {
    #[serde(rename = "meetingID")]
    pub meeting_id: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "attandeePW")]
    pub attandee_pw: Option<String>,

    #[serde(rename = "moderatorPW")]
    pub moderator_pw: Option<String>,

    #[serde(rename = "welcome")]
    pub welcome: Option<String>,

    #[serde(rename = "dialNumber")]
    pub dial_number: Option<String>,

    #[serde(rename = "voiceBridge")]
    pub voice_bridge: Option<String>,

    #[serde(rename = "maxParticipants")]
    pub max_participants: Option<u64>,

    #[serde(rename = "logoutURL")]
    pub logout_url: Option<String>,

    pub record: Option<bool>,

    pub duration: Option<u64>,

    #[serde(rename = "isBreakout")]
    pub is_breakout: Option<bool>,

    #[serde(rename = "parentMeetingID")]
    pub parent_meeting_id: Option<String>,

    pub sequence: Option<u64>,

    #[serde(rename = "freeJoin")]
    pub free_join: Option<bool>,

    pub meta: Option<String>,

    #[serde(rename = "moderatorOnlyMessage")]
    pub moderator_only_message: Option<String>,

    #[serde(rename = "autoStartRecording")]
    pub auto_start_recording: Option<bool>,

    #[serde(rename = "allowStartStopRecording")]
    pub allow_start_stop_recording: Option<bool>,

    #[serde(rename = "webcamsOnlyForModerator")]
    pub webcams_only_for_moderator: Option<bool>,

    pub logo: Option<String>,

    #[serde(rename = "bannerText")]
    pub banner_text: Option<String>,

    #[serde(rename = "bannerColor")]
    pub banner_color: Option<String>,

    pub copyright: Option<String>,

    #[serde(rename = "muteOnStart")]
    pub mute_on_start: Option<bool>,

    #[serde(rename = "allowModsToUnmuteUsers")]
    pub allow_mods_to_unmute_users: Option<bool>,

    #[serde(rename = "lockSettingsDisableCam")]
    pub lock_settings_disable_cam: Option<bool>,

    #[serde(rename = "lockSettingsDisableMic")]
    pub lock_settings_disable_mic: Option<bool>,

    #[serde(rename = "lockSettingsDisablePrivateChat")]
    pub lock_settings_disable_private_chat: Option<bool>,

    #[serde(rename = "lockSettingsDisablePublicChat")]
    pub lock_settings_disable_public_chat: Option<bool>,

    #[serde(rename = "lockSettingsDisableNote")]
    pub lock_settings_disable_note: Option<bool>,

    #[serde(rename = "lockSettingsLockedLayout")]
    pub lock_settings_locked_layout: Option<bool>,

    #[serde(rename = "lockSettingsLockOnJoin")]
    pub lock_settings_lock_on_join: Option<bool>,

    #[serde(rename = "lockSettingsLockOnJoinConfigurable")]
    pub lock_settings_lock_on_join_configurable: Option<bool>,

    #[serde(rename = "guestPolicy")]
    pub guest_policy: Option<String>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMeetingResponse {
    #[serde(rename = "returncode")]
    returncode: ErrorCode,

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
pub struct JoinMeetingRequest {
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,

    #[serde(rename = "meetingID")]
    pub meeting_id: Option<String>,

    #[serde(rename = "password")]
    pub password: Option<String>,

    #[serde(rename = "createTime")]
    pub create_time: Option<String>,

    #[serde(rename = "userID")]
    pub user_id: Option<String>,

    #[serde(rename = "webVoiceConf")]
    pub web_voice_conf: Option<String>,

    #[serde(rename = "configToken")]
    pub config_token: Option<String>,

    #[serde(rename = "defaultLayout")]
    pub default_layout: Option<u64>,

    #[serde(rename = "avatarURL")]
    pub avatar_url: Option<String>,

    redirect: bool,

    #[serde(rename = "clientURL")]
    pub client_url: Option<bool>,

    #[serde(rename = "joinViaHtml5")]
    pub join_via_html5: Option<u64>,

    pub guest: Option<bool>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JoinMeetingResponse {
    #[serde(rename = "returncode")]
    returncode: ErrorCode,

    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    message: Option<String>,

    meeting_id: Option<String>,

    user_id: Option<String>,

    auth_token: Option<String>,

    session_token: Option<String>,

    url: Option<String>,
}

impl CreateMeetingRequest {
    pub fn new() -> Self {
        Self {
            api_name: "create".to_string(),
            ..Default::default()
        }
    }
}
impl JoinMeetingRequest {
    pub fn new() -> Self {
        Self {
            api_name: "join".to_string(),
            redirect: true,
            ..Default::default()
        }
    }
}
use super::Bigbluebutton;
impl Bigbluebutton {
    pub async fn create_meeting(
        &self,
        request: &CreateMeetingRequest,
    ) -> Result<CreateMeetingResponse, BBBError> {
        let url = self.create_api_url(request);
        let text_response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let return_response;
        if text_response.contains("SUCCESS") {
            return_response =
                Ok(serde_xml_rs::from_str::<CreateMeetingResponse>(&text_response).unwrap());
        } else {
            return_response = Err(serde_xml_rs::from_str::<BBBError>(&text_response).unwrap());
        }
        return_response
    }
    pub async fn join_meeting(
        &self,
        request: &JoinMeetingRequest,
    ) -> Result<JoinMeetingResponse, BBBError> {
        let url = self.create_api_url(request);
        println!("{:?}", url);
        let text_response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        println!("{:?}", text_response);
        let return_response;
        if text_response.contains("SUCCESS") {
            return_response =
                Ok(serde_xml_rs::from_str::<JoinMeetingResponse>(&text_response).unwrap());
        } else {
            return_response = Err(serde_xml_rs::from_str::<BBBError>(&text_response).unwrap());
        }
        return_response
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

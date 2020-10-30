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
impl CreateMeetingRequest {
    pub fn new() -> Self {
        Self {
            api_name: "create".to_string(),
            ..Default::default()
        }
    }
}
pub trait GetApiName {
    fn get_query_params(&self) -> &str {
        "Hello"
    }
}
impl GetApiName for CreateMeetingRequest {
    fn get_query_params(&self) -> &str {
        &self.api_name
    }
}

use crate::error::ResponseCode;
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use bbb_macro::ApiName;
use getset::Getters;
use helper::GetApiName;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// Creates a new webhook
pub struct CreateHookRequest {
    #[serde(rename = "callbackURL")]
    /// The URL that will receive a POST call with the events. The same URL cannot be registered more than once.
    pub callback_url: String,

    #[serde(rename = "meetingID")]
    /// A meetingID to bind this hook to an specific meeting. If not informed, the hook will receive events for all meetings.
    pub meeting_id: Option<String>,

    #[serde(rename = "getRaw")]
    /// False by default. When getRaw=true, the POST call will contain the exact same message sent on redis, otherwise the message will be processed.
    pub get_raw: Option<bool>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Response return from [CreateHookRequest]
pub struct CreateHookResponse {
    #[serde(rename = "returncode")]
    /// SUCCESS of FAILED
    return_code: ResponseCode,

    /// Webhook id
    #[serde(rename = "hookID")]
    hook_id: String,

    /// Permament Hook Status
    #[serde(rename = "permanentHook")]
    permanent_hook: Option<bool>,

    /// Whether or not webhook events are processed or not
    #[serde(rename = "rawData")]
    raw_data: Option<bool>,

    /// Message key
    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    /// Message
    #[serde(rename = "message")]
    message: Option<String>,
}
impl CreateHookRequest {
    /// Creates new CreateHookRequest
    ///
    /// ```rust,no_run
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::webhook::CreateHookRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = CreateHookRequest::new("http://example.com/callback");
    /// bbb.execute(&request);
    /// ```
    pub fn new(callback_url: impl ToString) -> Self {
        Self {
            callback_url: callback_url.to_string(),
            api_name: "hooks/create".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<CreateHookRequest, CreateHookResponse> for Bigbluebutton {
    async fn execute(&self, request: &CreateHookRequest) -> anyhow::Result<CreateHookResponse> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// Removes hook. A hookID must be passed in the parameters to identify the hook to be removed.
pub struct DestroyHookRequest {
    #[serde(rename = "hookID")]
    /// The ID of the hook that should be removed, as returned in the create hook call.
    pub hook_id: String,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Response return from [DestroyHookRequest]
pub struct DestroyHookResponse {
    /// return code of meeting
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    /// If hook is removed or not
    removed: bool,
}
impl DestroyHookRequest {
    /// Creates new DestroyHookRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::webhook::DestroyHookRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = DestroyHookRequest::new("12");
    /// bbb.execute(&request);
    /// ```
    pub fn new(hook_id: impl ToString) -> Self {
        Self {
            hook_id: hook_id.to_string(),
            api_name: "hooks/destroy".to_string(),
        }
    }
}

#[async_trait]
impl Execute<DestroyHookRequest, DestroyHookResponse> for Bigbluebutton {
    async fn execute(&self, request: &DestroyHookRequest) -> anyhow::Result<DestroyHookResponse> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// Returns the hooks registered. If a meetingID is informed, will return the
/// hooks created specifically for this meeting plus all the global hooks
/// (since they also receive events for this meetingID). If no meetingID is informed,
/// returns all the hooks available (not only the global hooks, as might be expected).
pub struct ListHooksRequest {
    #[serde(rename = "meetingID")]
    /// A meeting ID to restrict the hooks returned only to the hooks that receive events for this meeting.
    pub meeting_id: Option<String>,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Response return from [ListHooksRequest]
pub struct ListHooksResponse {
    /// Return code Success or Failed
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    /// Hooks Details
    #[serde(deserialize_with = "from_hook")]
    hooks: Vec<Hook>,
}
#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Webhook Details
pub struct Hook {
    /// The ID of the webhook
    #[serde(rename = "hookID")]
    hook_id: String,

    /// The URL that will receive a POST call with the events. The same URL cannot be registered more than once.
    #[serde(rename = "callbackURL")]
    callback_url: String,

    /// MeetingId binded to webhook
    #[serde(rename = "meetingID")]
    meeting_id: Option<String>,

    /// Permanently stored hook status
    #[serde(rename = "permanentHook")]
    permanent_hook: bool,

    /// Whether or not data is processed
    #[serde(rename = "rawData")]
    raw_data: String,
}
fn from_hook<'de, D>(deserializer: D) -> Result<Vec<Hook>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct HookDetailsK {
        hook: Option<Vec<Hook>>,
    }

    let temp: HookDetailsK = Deserialize::deserialize(deserializer)?;
    if let Some(value) = temp.hook {
        Ok(value)
    } else {
        Ok(Vec::new())
    }
}
impl ListHooksRequest {
    /// Creates new ListHooksRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::webhook::ListHooksRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = ListHooksRequest::new();
    /// bbb.execute(&request);
    /// ```
    pub fn new() -> Self {
        Self {
            api_name: "hooks/list".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<ListHooksRequest, ListHooksResponse> for Bigbluebutton {
    async fn execute(&self, request: &ListHooksRequest) -> anyhow::Result<ListHooksResponse> {
        self.dispatch(request).await
    }
}

use crate::error::ResponseCode;
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use bbb_macro::ApiName;
use getset::Getters;
use helper::GetApiName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
/// Creates a BigBlueButton meeting.
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
    /// return code of meeting
    /// ```
    /// ```
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    #[serde(rename = "hookID")]
    hook_id: String,

    #[serde(rename = "permanentHook")]
    permanent_hook: Option<bool>,

    #[serde(rename = "rawData")]
    raw_data: Option<bool>,

    #[serde(rename = "messageKey")]
    message_key: Option<String>,

    #[serde(rename = "message")]
    message: Option<String>,
}
impl CreateHookRequest {
    /// Creates new CreateHookRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::webhook::CreateHookRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// //let mut request = CreateHookRequest::new("12");
    /// //bbb.execute(&request);
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
/// Response return from [CreateHookRequest]
pub struct DestroyHookResponse {
    /// return code of meeting
    /// ```
    /// ```
    #[serde(rename = "returncode")]
    return_code: ResponseCode,

    removed: bool,
}
impl DestroyHookRequest {
    /// Creates new DestoryHookRequest
    ///
    /// ```rust
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::webhook::DestoryHookRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// //let mut request = CreateHookRequest::new("12");
    /// //bbb.execute(&request);
    /// ```
    pub fn new(hook_id: impl ToString) -> Self {
        Self {
            hook_id: hook_id.to_string(),
            api_name: "hooks/destroy".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<DestroyHookRequest, DestroyHookResponse> for Bigbluebutton {
    async fn execute(&self, request: &DestoryHookRequest) -> anyhow::Result<DestroyHookResponse> {
        self.dispatch(request).await
    }
}

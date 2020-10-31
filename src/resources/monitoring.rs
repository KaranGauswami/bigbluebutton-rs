use crate::error::{BBBError, ErrorCode};
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
/// This call enables you to simply check on whether or not a meeting is running by looking it up with your meeting ID.
pub struct IsMeetingRunningRequest {
    #[serde(rename = "meetingID")]
    /// The meeting ID that identifies the meeting you are attempting to check on.
    pub meeting_id: Option<String>,
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response return from [IsMeetingRunningRequest]
pub struct IsMeetingRunningResponse {
    #[serde(rename = "returnCode")]
    return_code: Option<String>,

    running: Option<String>,
}
impl IsMeetingRunningRequest {
    /// Creates new IsMeetingRunningRequest
    pub fn new() -> Self {
        Self {
            api_name: "isMeetingRunning".to_string(),
            ..Default::default()
        }
    }
}

impl helper::GetApiName for IsMeetingRunningRequest {
    fn get_api_name(&self) -> &str {
        &self.api_name
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

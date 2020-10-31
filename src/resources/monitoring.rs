use crate::error::{BBBError, ErrorCode};
use crate::Bigbluebutton;
use crate::{helper, Execute};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IsMeetingRunningRequest {
    #[serde(rename = "meetingID")]
    pub meeting_id: Option<String>,
    pub password: Option<String>,
    api_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IsMeetingRunningResponse {
    #[serde(rename = "returnCode")]
    return_code: Option<String>,

    running: Option<String>,
}
impl IsMeetingRunningRequest {
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

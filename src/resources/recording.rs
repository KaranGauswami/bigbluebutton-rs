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
pub struct PublishRecordingsRequest {
    #[serde(rename = "recordID")]
    /// The URL that will receive a POST call with the events. The same URL cannot be registered more than once.
    pub record_id: String,

    /// A meetingID to bind this hook to an specific meeting. If not informed, the hook will receive events for all meetings.
    pub publish: bool,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Response return from [PublishRecordingsResponse]
pub struct PublishRecordingsResponse {
    #[serde(rename = "returncode")]
    /// return code
    return_code: ResponseCode,

    /// published status of recording
    published: bool,
}
impl PublishRecordingsRequest {
    /// Creates new PublishRecordingsRequest
    ///
    /// ```rust,no_run
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::recording::PublishRecordingsRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = PublishRecordingsRequest::new("12",false);
    /// bbb.execute(&request);
    /// ```
    pub fn new(record_id: impl ToString, publish: bool) -> Self {
        Self {
            record_id: record_id.to_string(),
            publish,
            api_name: "publishRecordings".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<PublishRecordingsRequest, PublishRecordingsResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &PublishRecordingsRequest,
    ) -> anyhow::Result<PublishRecordingsResponse> {
        self.dispatch(request).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ApiName)]
///Delete one or more recordings for a given recordID (or set of record IDs).

pub struct DeleteRecordingsRequest {
    #[serde(rename = "recordID")]
    /// A record ID for specify the recordings to delete. It can be a set of record IDs separated by commas.
    pub record_id: String,

    #[serde(skip)]
    api_name: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
/// Response return from [DeleteRecordingsRequest]
pub struct DeleteRecordingsResponse {
    #[serde(rename = "returncode")]
    /// return code
    return_code: ResponseCode,

    /// whether recording is deleted or not
    deleted: bool,
}
impl DeleteRecordingsRequest {
    /// Creates new DeleteRecordingsRequest
    ///
    /// ```rust,no_run
    /// # use bigbluebutton::{Bigbluebutton,Execute};
    /// use bigbluebutton::recording::DeleteRecordingsRequest;
    /// let bbb = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = DeleteRecordingsRequest::new("12",false);
    /// bbb.execute(&request);
    /// ```
    pub fn new(record_id: impl ToString) -> Self {
        Self {
            record_id: record_id.to_string(),
            api_name: "deleteRecordings".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Execute<DeleteRecordingsRequest, DeleteRecordingsResponse> for Bigbluebutton {
    async fn execute(
        &self,
        request: &DeleteRecordingsRequest,
    ) -> anyhow::Result<DeleteRecordingsResponse> {
        self.dispatch(request).await
    }
}

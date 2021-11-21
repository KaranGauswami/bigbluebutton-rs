use crate::error::ResponseCode;
use crate::Bigbluebutton;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Creates a BigBlueButton meeting.
pub struct PublishRecordingsRequest {
    #[serde(rename = "recordID")]
    /// The URL that will receive a POST call with the events. The same URL cannot be registered more than once.
    pub record_id: String,

    /// A meetingID to bind this hook to an specific meeting. If not informed, the hook will receive events for all meetings.
    pub publish: bool,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
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
    /// # use bigbluebutton::Bigbluebutton;
    /// use bigbluebutton::recording::PublishRecordingsRequest;
    /// let client = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = PublishRecordingsRequest::new("12", false);
    /// client.publish_recordings(&request);
    /// ```
    pub fn new(record_id: impl ToString, publish: bool) -> Self {
        Self {
            record_id: record_id.to_string(),
            publish,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
///Delete one or more recordings for a given recordID (or set of record IDs).

pub struct DeleteRecordingsRequest {
    #[serde(rename = "recordID")]
    /// A record ID for specify the recordings to delete. It can be a set of record IDs separated by commas.
    pub record_id: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
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
    /// # use bigbluebutton::Bigbluebutton;
    /// use bigbluebutton::recording::DeleteRecordingsRequest;
    /// let client = Bigbluebutton::new("https://server.com/bigbluebutton/", "secret");
    /// let mut request = DeleteRecordingsRequest::new("12");
    /// client.delete_recordings(&request);
    /// ```
    pub fn new(record_id: impl ToString) -> Self {
        Self {
            record_id: record_id.to_string(),
        }
    }
}

impl Bigbluebutton {
    pub async fn delete_recordings(
        &self,
        req: &DeleteRecordingsRequest,
    ) -> Result<DeleteRecordingsResponse, anyhow::Error> {
        self.dispatch("deleteRecordings", req).await
    }
    pub async fn publish_recordings(
        &self,
        req: &PublishRecordingsRequest,
    ) -> Result<PublishRecordingsResponse, anyhow::Error> {
        self.dispatch("publishRecordings", req).await
    }
}

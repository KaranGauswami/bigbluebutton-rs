use std::collections::HashMap;

mod helper;

pub struct Bigbluebutton {
    salt: String,
    url: String,
}
impl Bigbluebutton {
    /// creates new BBB API Client
    pub fn new(url: String, salt: String) -> Self {
        let mut api_url = url;
        api_url.push_str("api/");
        Self { salt, url: api_url }
    }

    /// Generates BBB URL
    pub fn generate_url(self, action: String, params: HashMap<String, String>) -> String {
        let query_params = self::Bigbluebutton::serialize_params(params);
        let checksum =
            self::Bigbluebutton::hash(vec![action.clone(), query_params.clone(), self.salt]);
        format!(
            "{}{}?{}&checksum={}",
            self.url, action, query_params, checksum
        )
    }

    /// Creates a BigBlueButton meeting.
    ///
    /// The create call is idempotent: you can call it multiple times with the same parameters
    /// without side effects.
    /// This simplifies the logic for joining a user into a session as your application can always
    /// call create before returning the join URL to the user. This way, regardless of the order in
    /// which users join, the meeting will always exist when the user tries to join (the first create
    /// call actually creates the meeting; subsequent calls to create simply return SUCCESS).
    /// The BigBlueButton server will automatically remove empty meetings that were created but have
    /// never had any users after a number of minutes specified by meetingExpireIfNoUserJoinedInMinutes
    /// defined in bigbluebutton.properties.
    pub fn create(self, params: HashMap<String, String>) {
        unimplemented!();
    }
}

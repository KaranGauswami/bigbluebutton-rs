use std::collections::HashMap;

use bigbluebutton::Bigbluebutton;

fn main() {
    // Creates new BBB Instance
    let client = Bigbluebutton::new("https://example.com/bigbluebutton/", "BBBSECRET");

    let mut params = HashMap::new();
    params.insert("password", "pass");
    params.insert("fullName", "name");
    params.insert("meetingID", "1");

    let url = client
        .generate_url("join", params)
        .expect("unable to generate url");

    println!("{}", url); // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingID=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfd
}

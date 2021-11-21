use bigbluebutton::Bigbluebutton;

fn main() {
    // Creates new BBB Instance
    let client = Bigbluebutton::new("https://example.com/bigbluebutton/", "BBBSECRET");

    let params = vec![
        ("password", "pass"),
        ("fullName", "name"),
        ("meetingId", "1"),
    ];

    let url = client.generate_url("join", params);

    println!("{}", url) // https://example.com/bigbluebutton/api/join?password=pass&fullName=name&meetingId=1&checksum=94e467c1b4b13f4452ca5d1deb9b7b74e1063aea55fe078139015a7d6311cfd
}

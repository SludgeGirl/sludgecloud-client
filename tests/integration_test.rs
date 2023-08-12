use sludgecloud::{Client, Credentials};

#[test]
fn test_client() {
    let mut client = Client::new(Some(Credentials {
        username: "".into(),
        secret: "".into(),
    }));

    client.upload_file("/home/sludge/testing".into());
}

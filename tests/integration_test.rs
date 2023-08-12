use sludgecloud::{Client, Credentials};

#[test]
fn test_client() {
    let mut client = Client::new(Some(Credentials {
        username: "",
        secret: "",
    }));

    client.upload_file("/home/sludge/testing");
}

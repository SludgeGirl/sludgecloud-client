use crate::Credentials;

#[derive(Clone)]
pub struct Config {
    credentials: Option<Credentials>,
    host: String,
}

impl Config {
    pub fn new(host: String) -> Self {
        Config {
            credentials: None,
            host,
        }
    }
}

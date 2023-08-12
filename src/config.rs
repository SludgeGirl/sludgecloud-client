use crate::Credentials;

#[derive(Clone)]
pub struct Config<'a> {
    credentials: Option<Credentials<'a>>,
    host: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(host: &'a str) -> Self {
        Config {
            credentials: None,
            host,
        }
    }
}

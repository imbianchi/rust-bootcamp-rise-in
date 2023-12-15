pub struct Helpers {}

#[derive(Debug)]
pub struct Credentials {
    username: String,
    pswd: String,
}

impl Helpers {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_credentials(&self) -> Option<Credentials> {
        unimplemented!()
    }

    pub fn create_config_file(&self) {
        ()
    }
}

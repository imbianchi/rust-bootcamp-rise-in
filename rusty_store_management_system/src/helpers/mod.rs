use std::{
    fs::File,
    io::{Error, Read},
};

use serde_json::Value;

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

    pub fn config_file_exists() -> (Self, bool) {
        if std::path::Path::new("config.json").exists() {
            return (Self {}, true);
        } else {
            return (Self {}, false);
        }
    }

    pub fn read_config_file(&self) -> String {
        let mut file = File::open("config.json").expect("Failed to open file");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");

        content
    }

    pub fn get_credentials(&self) -> Option<Credentials> {
        let file_content = self.read_config_file();

        let json_value: Value =
            serde_json::from_str(&file_content).expect("Error getting JSON content.");
        let mut username = String::from("");
        let mut pswd = String::from("");

        if let Value::Object(obj) = json_value {
            // Iterate through key-value pairs
            for (key, value) in obj {
                if key == "username" {
                    username = value.to_string();
                }

                if key == "pswd" {
                    pswd = value.to_string();
                }
            }
        } else {
            println!("The JSON file does not contain an object at the top level.");
            return None;
        }

        Some(Credentials { username, pswd })
    }

    pub fn create_config_file(&self) {
        ()
    }
}

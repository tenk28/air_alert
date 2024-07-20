use log::error;
use std::fs::File;
use std::io::Read;

const CONFIG_PATHNAME: &str = "config.json";
const PLACEHOLDER_API_KEY: &str = "<your api key here>";

const API_KEY_FIELD: &str = "apiKey";
const REGION_ID_FIELD: &str = "regionId";

pub struct Config {
    api_key: String,
    region_id: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            api_key: String::default(),
            region_id: String::default(),
        }
    }

    pub fn get_api_key(&self) -> &String {
        if self.api_key.is_empty() {
            error!("api_key is empty, did you run parse?");
        }
        &self.api_key
    }

    pub fn get_regions_id(&self) -> &String {
        if self.region_id.is_empty() {
            error!("region_id is empty, did you run parse?");
        }
        &self.region_id
    }

    pub fn parse(&mut self) -> bool {
        let mut file = match File::open(CONFIG_PATHNAME) {
            Ok(file) => file,
            Err(err) => {
                error!("Failed to open file: {}", err);
                return false;
            }
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to read file content: {}", err);
                return false;
            }
        }

        let parsed = match json::parse(content.as_str()) {
            Ok(parsed) => parsed,
            Err(err) => {
                error!("Failed to parse content: {}", err);
                return false;
            }
        };

        let api_key_obj = &parsed[API_KEY_FIELD];
        if api_key_obj.is_null() {
            error!("Failed to parse content: {} field is null", API_KEY_FIELD);
            return false;
        }
        if api_key_obj == PLACEHOLDER_API_KEY {
            error!(
                "Failed to parse content: fill {} with your key",
                API_KEY_FIELD
            );
            return false;
        }
        self.api_key = api_key_obj.to_string();

        let region_id_obj = &parsed[REGION_ID_FIELD];
        if region_id_obj.is_null() {
            error!("Failed to parse content: {} field is null", REGION_ID_FIELD);
            return false;
        }
        self.region_id = region_id_obj.to_string();

        true
    }
}

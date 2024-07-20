use json::{self, JsonValue};
use std::fs::File;
use std::io::Read;

const CONFIG_PATHNAME: &str = "config.json";
const PLACEHOLDER_API_KEY: &str = "<your api key here>";

const API_KEY_FIELD: &str = "apiKey";
const REGIONS_TO_OBSERVE_FIELD: &str = "regionsToObserve";
const REGIONS_ID_FIELD: &str = "regionId";

pub struct Config {
    api_key: String,
    regions_id: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            api_key: String::default(),
            regions_id: Vec::default(),
        };
        let content = config.read_config();
        config.parse_content(&content);
        config
    }

    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn get_regions_id(&self) -> &Vec<String> {
        &self.regions_id
    }

    fn read_config(&mut self) -> String {
        let mut file = match File::open(CONFIG_PATHNAME) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {}", err);
                return "".to_string();
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to read file content: {}", err);
                return "".to_string();
            }
        }

        contents
    }

    fn parse_content(&mut self, content: &str) {
        let parsed = match json::parse(content) {
            Ok(parsed) => parsed,
            Err(err) => {
                eprintln!("Failed to parse content: {}", err);
                return;
            }
        };

        let api_key_obj = &parsed[API_KEY_FIELD];
        if api_key_obj.is_null() {
            eprintln!("Failed to parse content: {} field is null", API_KEY_FIELD);
            return;
        }
        if api_key_obj == PLACEHOLDER_API_KEY {
            eprintln!(
                "Failed to parse content: fill {} with your key",
                API_KEY_FIELD
            );
            return;
        }
        self.api_key = api_key_obj.to_string();

        let regions_to_observe = &parsed[REGIONS_TO_OBSERVE_FIELD];
        if regions_to_observe.is_null() {
            eprintln!(
                "Failed to parse content: {} field is null",
                REGIONS_TO_OBSERVE_FIELD
            );
            return;
        }
        let regions_id_filter_func = |obj: &JsonValue| {
            if !obj[REGIONS_ID_FIELD].is_null() {
                Some(obj[REGIONS_ID_FIELD].to_string())
            } else {
                None
            }
        };
        self.regions_id = regions_to_observe
            .members()
            .filter_map(regions_id_filter_func)
            .collect();
    }
}

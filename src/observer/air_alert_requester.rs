use log::error;
use reqwest::{
    self,
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    StatusCode,
};

const AIR_ALERT_MAIN_URI: &str = "https://api.ukrainealarm.com/api/v3/alerts/";
const ACTIVE_ALERTS_FIELD: &str = "activeAlerts";

pub struct AirAlertRequester {
    api_key: String,
}

impl AirAlertRequester {
    pub fn new(api_key: &str) -> Self {
        AirAlertRequester {
            api_key: api_key.to_string(),
        }
    }

    pub fn is_alert_in_region(&self, region_id: &str) -> Option<bool> {
        let url = String::from(AIR_ALERT_MAIN_URI) + region_id;

        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));
        let authorization_value = match HeaderValue::from_str(&self.api_key) {
            Ok(authorization_value) => authorization_value,
            Err(err) => {
                error!("Failed to get str from api_key: {}", err);
                return None;
            }
        };

        headers.insert("Authorization", authorization_value);

        let response = match client.get(url.as_str()).headers(headers).send() {
            Ok(resp) => resp,
            Err(err) => {
                error!("Failed to GET {}: {}", url, err);
                return None;
            }
        };

        let status_code = response.status();
        if status_code != StatusCode::OK {
            error!("Response error, status code: {}", status_code.as_str());
            return None;
        }

        let content = match response.text() {
            Ok(content) => content,
            Err(err) => {
                error!("Failed to get response text: {}", err);
                return None;
            }
        };

        let parsed = match json::parse(&content) {
            Ok(parsed) => parsed,
            Err(err) => {
                error!("Failed to parse content: {}", err);
                error!("content: {}", &content);
                return None;
            }
        };

        if parsed[0][ACTIVE_ALERTS_FIELD].is_null() {
            error!("Field \"{}\" is null", ACTIVE_ALERTS_FIELD);
            return None;
        }

        if (parsed[0][ACTIVE_ALERTS_FIELD]).is_empty() {
            Some(false)
        } else {
            Some(true)
        }
    }
}

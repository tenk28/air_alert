mod air_alert_requester;

use air_alert_requester::AirAlertRequester;
use log::info;
use std::{thread, time::Duration};

const OBSERVE_DELAY_SECONDS: u64 = 35;

pub struct Observer {
    air_alert_requester: AirAlertRequester,
    region_id: String,
}

impl Observer {
    pub fn new(api_key: &str, region_id: &str) -> Self {
        Observer {
            air_alert_requester: AirAlertRequester::new(api_key),
            region_id: region_id.to_string(),
        }
    }

    pub fn poll(&mut self) {
        loop {
            if let Some(is_alert) = self
                .air_alert_requester
                .is_alert_in_region(self.region_id.as_str())
            {
                if is_alert {
                    info!("There is alert in {} region", self.region_id);
                } else {
                    info!("There is no alert in {} region", self.region_id);
                }

                thread::sleep(Duration::from_secs(OBSERVE_DELAY_SECONDS));
            }
        }
    }
}

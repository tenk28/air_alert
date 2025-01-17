mod air_alert_requester;
mod audioer;

use air_alert_requester::AirAlertRequester;
use audioer::play_audio;
use log::{error, info, debug};
use std::{thread, time::Duration};

const OBSERVE_DELAY_SECONDS: u64 = 30;
const PLAY_COUNT: u64 = 7;

pub struct Observer {
    air_alert_requester: AirAlertRequester,
    region_id: String,
    is_alert: bool,
    air_alert_start_audio_pathname: String,
    air_alert_end_audio_pathname: String,
}

impl Observer {
    pub fn new(
        api_key: &str,
        region_id: &str,
        air_alert_start_audio: &str,
        air_alert_end_audio: &str,
    ) -> Self {
        let air_alert_requester: AirAlertRequester = AirAlertRequester::new(api_key);

        let is_alert = loop {
            match air_alert_requester.is_alert_in_region(region_id) {
                Some(air_alert_value) => {
                    info!("Observing \"{}\" region, init alert value \"{}\"", air_alert_value.region_eng_name, air_alert_value.is_alert.to_string());
                    break air_alert_value.is_alert;
                }
                None => {
                    error!("Failed to get init values of region_id: {}", region_id);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        };
        

        Observer {
            air_alert_requester,
            region_id: region_id.to_string(),
            is_alert,
            air_alert_start_audio_pathname: air_alert_start_audio.to_string(),
            air_alert_end_audio_pathname: air_alert_end_audio.to_string(),
        }
    }

    pub fn poll(&mut self) {
        loop {
            if let Some(air_alert_value) = self
                .air_alert_requester
                .is_alert_in_region(self.region_id.as_str())
            {
                if air_alert_value.is_alert != self.is_alert {
                    let audio_pathname;
                    if air_alert_value.is_alert {
                        audio_pathname = &self.air_alert_start_audio_pathname;
                        info!(
                            "There is alert in {} region",
                            air_alert_value.region_eng_name
                        );
                    } else {
                        audio_pathname = &self.air_alert_end_audio_pathname;
                        info!(
                            "There is no alert in {} region",
                            air_alert_value.region_eng_name
                        );
                    }
                    for _ in 0..PLAY_COUNT {
                        play_audio(audio_pathname);
                    }
                }
                self.is_alert = air_alert_value.is_alert;
                debug!("Alert status \"{}\" in \"{}\" region", air_alert_value.is_alert.to_string(), air_alert_value.region_eng_name);
            }
            thread::sleep(Duration::from_secs(OBSERVE_DELAY_SECONDS));
        }
    }
}

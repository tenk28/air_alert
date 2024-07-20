mod air_alert_requester;
mod configer;

use crate::air_alert_requester::AirAlertRequester;
use crate::configer::Config;

fn main() {
    let config = Config::new();
    let air_alert_requester = AirAlertRequester::new(config.get_api_key());
    println!(
        "{}",
        air_alert_requester.is_alert_in_region("9999").unwrap()
    );
}

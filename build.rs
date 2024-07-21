use std::{fs, env};

const BINARY_NAME: &str = "air_alert";

const SERVICE_DESTINATION: &str = "/etc/systemd/system/";

const RESOURCES_DIR: &str = "rsc";
const SAMPLES_DIR: &str = "samples";
const LOGS_DIR: &str = "/var/air_alert";

const SERVICE_FILENAME: &str = "air_alert.service";
const CONFIG_FILENAME: &str = "config.json";
const START_ALERT_FILENAME: &str = "start_air_alert.mp3";
const END_ALERT_FILENAME: &str = "end_air_alert.mp3";

fn main() {
    let cargo_home = env::var("CARGO_HOME").unwrap_or_else(|_| {
        // If CARGO_HOME is not set, default to the usual location
        let home_dir = env::var("HOME").expect("HOME environment variable is not set");
        format!("{}/.cargo", home_dir)
    });

    let service_file_from = format!("{SAMPLES_DIR}/{SERVICE_FILENAME}");
    let config_file_from = format!("{SAMPLES_DIR}/{CONFIG_FILENAME}");
    let start_alert_audio_from = format!("{RESOURCES_DIR}/{START_ALERT_FILENAME}");
    let end_alert_audio_from = format!("{RESOURCES_DIR}/{END_ALERT_FILENAME}");

    let service_file_to = format!("{SERVICE_DESTINATION}/{SERVICE_FILENAME}") ;

    let config_dir_to = dirs::config_dir().unwrap().join(BINARY_NAME);
    let rsc_dir_to = config_dir_to.join(RESOURCES_DIR);
    
    let config_file_to = format!("{}/{CONFIG_FILENAME}", config_dir_to.to_str().unwrap());
    let start_alert_audio_to = format!("{}/{START_ALERT_FILENAME}", rsc_dir_to.to_str().unwrap());
    let end_alert_audio_to = format!("{}/{END_ALERT_FILENAME}", rsc_dir_to.to_str().unwrap());
    let binary_pathname_to = format!("{cargo_home}/bin/{BINARY_NAME}");
    
    let user = env::var("USER").expect("USER environment variable not set");

    let parsed_service_file = fs::read_to_string(service_file_from).expect("Unable to read template file");
    let parsed_service_file = parsed_service_file.replace("{{$binary_pathname}}", binary_pathname_to.as_str());
    let parsed_service_file = parsed_service_file.replace("{{$config_pathname}}", config_file_to.as_str());
    let parsed_service_file = parsed_service_file.replace("{{$start_alert_pathname}}", start_alert_audio_to.as_str());
    let parsed_service_file = parsed_service_file.replace("{{$end_alert_pathname}}", end_alert_audio_to.as_str());
    let parsed_service_file = parsed_service_file.replace("{{$logs_dir}}", LOGS_DIR);
    let parsed_service_file = parsed_service_file.replace("{{$user}}", user.as_str());

    fs::write(service_file_to.as_str(), parsed_service_file).expect(format!("Failed to write to \"{}\"", service_file_to).as_str());

    if let Ok(_) = fs::create_dir_all(&rsc_dir_to) {
        fs::copy(config_file_from, config_file_to).unwrap();
        fs::copy(start_alert_audio_from, start_alert_audio_to).unwrap();
        fs::copy(end_alert_audio_from, end_alert_audio_to).unwrap();
    }
    fs::create_dir_all(LOGS_DIR).expect(format!("Failed to create \"{}\"", LOGS_DIR).as_str());
}

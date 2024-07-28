mod configer;
mod logger;
mod observer;

use clap::{Arg, ArgMatches, Command};
use configer::Config;
use log::info;
use logger::init_logger;
use observer::Observer;

const CONFIG_PATHNAME: &str = "samples/config.json";
const START_ALERT_AUDIO_PATHNAME: &str = "rsc/start_air_alert.mp3";
const END_ALERT_AUDIO_PATHNAME: &str = "rsc/end_air_alert.mp3";
const LOGS_DIR: &str = "logs";

fn main() {
    let cli = cli();
    init_logger(cli.get_one::<String>("logs_dir").unwrap());
    info!("Air Alert app started");

    let mut config = Config::new(cli.get_one::<String>("config_file").unwrap());
    if config.parse() {
        let mut observer = Observer::new(
            config.get_api_key(),
            config.get_regions_id(),
            cli.get_one::<String>("start_alert_audio").unwrap(),
            cli.get_one::<String>("end_alert_audio").unwrap(),
        );
        observer.poll();
    }
}

fn cli() -> ArgMatches {
    Command::new("air_alert")
        .author("Dmytro Shtrikker, shtrikker28@gmail.com")
        .about("Tracks air alert in your region and notifies with audio samples")
        .arg(
            Arg::new("config_file")
                .long("config_file")
                .default_value(CONFIG_PATHNAME)
                .help("pathname to config file"),
        )
        .arg(
            Arg::new("start_alert_audio")
                .long("start_alert_audio")
                .default_value(START_ALERT_AUDIO_PATHNAME)
                .help("pathname to start alert audio sample"),
        )
        .arg(
            Arg::new("end_alert_audio")
                .long("end_alert_audio")
                .default_value(END_ALERT_AUDIO_PATHNAME)
                .help("pathname to end alert audio sample"),
        )
        .arg(
            Arg::new("logs_dir")
                .long("logs_dir")
                .default_value(LOGS_DIR)
                .help("directory, where to store logs"),
        )
        .get_matches()
}

mod configer;
mod logger;
mod observer;

use configer::Config;
use log::info;
use logger::init_logger;
use observer::Observer;

fn main() {
    init_logger();

    info!("Air Alert app started");

    let mut config = Config::new();
    if config.parse() {
        let mut observer = Observer::new(config.get_api_key(), config.get_regions_id());
        observer.poll();
    }
}

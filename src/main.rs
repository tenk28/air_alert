mod configer;
mod logger;
mod observer;

use configer::Config;
use logger::init_logger;
use observer::Observer;

fn main() {
    init_logger();

    let mut config = Config::new();
    if config.parse() {
        let mut observer = Observer::new(config.get_api_key(), config.get_regions_id());
        observer.poll();
    }
}

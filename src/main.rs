mod configer;
mod observer;

use configer::Config;
use observer::Observer;

fn main() {
    let mut config = Config::new();
    if config.parse() {
        let mut observer = Observer::new(config.get_api_key(), config.get_regions_id());
        observer.poll();
    }
}

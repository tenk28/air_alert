mod configer;
mod observer;

use configer::Config;
use observer::Observer;

fn main() {
    let config = Config::new();
    let mut observer = Observer::new(config.get_api_key(), config.get_regions_id());
    observer.poll();
}

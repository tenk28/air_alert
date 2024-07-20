mod configer;

use crate::configer::Config;

fn main() {
    let config = Config::new();
    println!("my api key: {}", config.get_api_key());
    println!("my regions key: {:?}", config.get_regions_id());
}

// aardwolf-api/backend-api/src/backend_main.rs
use aardwolf_api_common::config::load_config;

fn main() {
    let settings = load_config().expect("Failed to load config");
    println!("Using backend engine: {}", settings.backend_engine);
}

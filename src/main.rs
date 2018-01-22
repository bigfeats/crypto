extern crate crypto;
use crypto::config::Config;

fn main() {
    match Config::new() {
        Ok(config) => crypto::do_crypto(&config),
        Err(message) => println!("ERROR: {}", message)
    }
}


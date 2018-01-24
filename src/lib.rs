pub mod config;
pub mod algorithms;

pub fn do_crypto(config: &config::Config) {
    let result;
    if config.encrypt {
        result = algorithms::encrypt(&config.algorithm, &config.message, &config.key);
    } else {
        result = algorithms::decrypt(&config.algorithm, &config.message, &config.key);
    }

    match result {
        Ok(result_str) => println!("{}", result_str),
        Err(error_str) => eprintln!("Encryption error: {}", error_str)
    }
}

extern crate clap;

use self::clap::{Arg, App};

#[derive(Debug)]
pub struct Config {
    pub encrypt: bool,    // encrypt or decrypt?
    pub key: String,      // the key, could be numeric or a string, but let it be parsed later.
    pub message: String,  // the message to encrypt or decrypt, assumed ascii for now.
    pub filename: String, // the file which stores the message to encrypt or decrypt.
    pub algorithm: String // which algorithm is being used?
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        // Example usage: ./crypto -e hill -k key -f filename.plain
        // Example usage: ./crypto -d hill -k key -f filename.cipher
        // Example usage: ./crypto -e hill -k key -m "This is a message to encrypt"

        let matches = App::new("Crypto Tool")
            .arg(Arg::with_name("encrypt")
                .short("e")
                .takes_value(true))
            .arg(Arg::with_name("decrypt")
                .short("d")
                .takes_value(true))
            .arg(Arg::with_name("key")
                .short("k")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("message")
                .short("m")
                .takes_value(true))
            .arg(Arg::with_name("filename")
                .short("f")
                .takes_value(true))
            .get_matches();

        let key = String::from(matches.value_of("key").unwrap()); // key is required, unwrap is safe
        let message = String::from(matches.value_of("message").unwrap_or(""));
        let filename = String::from(matches.value_of("filename").unwrap_or(""));

        let encrypt_algorithm = matches.value_of("encrypt").unwrap_or("");
        let decrypt_algorithm = matches.value_of("decrypt").unwrap_or("");

        let encrypt;
        let algorithm;
        if encrypt_algorithm != "" {
            encrypt = true;
            algorithm = String::from(encrypt_algorithm);
        } else if decrypt_algorithm != "" {
            encrypt = false;
            algorithm = String::from(decrypt_algorithm);
        } else {
            return Err("Must define encrypt or decrypt");
        }

        Ok(Config { encrypt, key, message, filename, algorithm })
    }
}

use crate::handlers;
use clap::ArgMatches;

pub struct Command {
    pub name: String,
    pub help_msg: String,
    pub handler: fn(&ArgMatches),
}
impl Command {
    fn new_checksum(name: &str, handler: fn(&ArgMatches)) -> Self {
        Self {
            name: name.to_owned(),
            help_msg: format!("Get file {} checksum.", name),
            handler,
        }
    }
    fn new_cipher(name: &str, handler: fn(&ArgMatches)) -> Self {
        Self {
            name: name.to_owned(),
            help_msg: format!("Encrypt file with {}.", name),
            handler,
        }
    }
}

pub struct Config {
    pub checksums: Vec<Command>,
    pub ciphers: Vec<Command>,
}

pub fn init() -> Config {
    Config {
        checksums: [
            ("md5", handlers::md5_handler as fn(&ArgMatches)),
            ("sha1", handlers::sha1_handler),
            ("sha2", handlers::sha2_handler),
            ("sha3", handlers::sha3_handler),
        ]
        .iter()
        .map(|(n, handler)| Command::new_checksum(n, *handler))
        .collect(),
        ciphers: [("aes", handlers::aes_handler as fn(&ArgMatches))]
            .iter()
            .map(|(n, handler)| Command::new_cipher(n, *handler))
            .collect(),
    }
}

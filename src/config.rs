use crate::handlers;
use clap::ArgMatches;

pub struct Command {
    pub name: String,
    pub help_msg: String,
    pub handler: fn(&ArgMatches) -> Result<(), failure::Error>,
}
impl Command {
    fn new_checksum(name: &str, handler: fn(&ArgMatches) -> Result<(), failure::Error>) -> Self {
        Self {
            name: name.to_owned(),
            help_msg: format!("Get {} checksum.", name),
            handler,
        }
    }
    fn new_cipher(name: &str, handler: fn(&ArgMatches) -> Result<(), failure::Error>) -> Self {
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
            (
                "md5",
                handlers::md5_handler as fn(&ArgMatches) -> Result<(), failure::Error>,
            ),
            ("sha1", handlers::sha1_handler),
            ("sha2-224", handlers::sha2_224_handler),
            ("sha2-256", handlers::sha2_256_handler),
            ("sha2-384", handlers::sha2_384_handler),
            ("sha2-512", handlers::sha2_512_handler),
            ("sha3-224", handlers::sha3_224_handler),
            ("sha3-256", handlers::sha3_256_handler),
            ("sha3-384", handlers::sha3_384_handler),
            ("sha3-512", handlers::sha3_512_handler),
        ]
        .iter()
        .map(|(n, handler)| Command::new_checksum(n, *handler))
        .collect(),
        ciphers: [(
            "aes",
            handlers::aes_handler as fn(&ArgMatches) -> Result<(), failure::Error>,
        )]
        .iter()
        .map(|(n, handler)| Command::new_cipher(n, *handler))
        .collect(),
    }
}

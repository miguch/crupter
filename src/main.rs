mod args;
mod ciphers;
mod handlers;
mod hashes;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::Write;

use args::{CipherArgs, HashArgs};

#[derive(Parser)]
#[command(
    name = "Crupter",
    author = "Miguel Chan <i@miguch.com>",
    about = "An easy tool for file hash and encryption",
    subcommand_required = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Get md5 checksum
    #[command(name = "md5")]
    Md5(HashArgs),
    /// Get sha1 checksum
    #[command(name = "sha1")]
    Sha1(HashArgs),
    /// Get sha2-224 checksum
    #[command(name = "sha2-224")]
    Sha2224(HashArgs),
    /// Get sha2-256 checksum
    #[command(name = "sha2-256")]
    Sha2256(HashArgs),
    /// Get sha2-384 checksum
    #[command(name = "sha2-384")]
    Sha2384(HashArgs),
    /// Get sha2-512 checksum
    #[command(name = "sha2-512")]
    Sha2512(HashArgs),
    /// Get sha3-224 checksum
    #[command(name = "sha3-224")]
    Sha3224(HashArgs),
    /// Get sha3-256 checksum
    #[command(name = "sha3-256")]
    Sha3256(HashArgs),
    /// Get sha3-384 checksum
    #[command(name = "sha3-384")]
    Sha3384(HashArgs),
    /// Get sha3-512 checksum
    #[command(name = "sha3-512")]
    Sha3512(HashArgs),
    /// Encrypt file with aes-128
    #[command(name = "aes-128")]
    Aes128(CipherArgs),
    /// Encrypt file with aes-192
    #[command(name = "aes-192")]
    Aes192(CipherArgs),
    /// Encrypt file with aes-256
    #[command(name = "aes-256")]
    Aes256(CipherArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Md5(args) => handlers::md5_handler(args),
        Command::Sha1(args) => handlers::sha1_handler(args),
        Command::Sha2224(args) => handlers::sha2_224_handler(args),
        Command::Sha2256(args) => handlers::sha2_256_handler(args),
        Command::Sha2384(args) => handlers::sha2_384_handler(args),
        Command::Sha2512(args) => handlers::sha2_512_handler(args),
        Command::Sha3224(args) => handlers::sha3_224_handler(args),
        Command::Sha3256(args) => handlers::sha3_256_handler(args),
        Command::Sha3384(args) => handlers::sha3_384_handler(args),
        Command::Sha3512(args) => handlers::sha3_512_handler(args),
        Command::Aes128(args) => handlers::aes_128_handler(args),
        Command::Aes192(args) => handlers::aes_192_handler(args),
        Command::Aes256(args) => handlers::aes_256_handler(args),
    };
    if let Err(ref e) = result {
        writeln!(&mut std::io::stderr(), "{}", e).unwrap();
    }
    result
}

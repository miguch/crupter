use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct HashArgs {
    #[arg(value_name = "FILE")]
    pub filenames: Vec<PathBuf>,
    #[arg(short = 'j', long, default_value_t = 0)]
    pub parallels: u32,
    #[arg(short = 's', long)]
    pub silent: bool,
}

#[derive(Args, Debug)]
pub struct CipherArgs {
    #[arg(short = 'p', long, value_name = "STRING")]
    pub passphrase: String,
    #[arg(value_name = "FILE")]
    pub filenames: Vec<PathBuf>,
    #[arg(short = 'o', long, value_name = "PATH", default_value = "")]
    pub output: String,
    #[arg(short = 'd', long)]
    pub decrypt: bool,
    #[arg(short = 'j', long, default_value_t = 0)]
    pub parallels: u32,
    #[arg(short = 's', long)]
    pub silent: bool,
    #[arg(short = 'l', long)]
    pub list_name: bool,
}

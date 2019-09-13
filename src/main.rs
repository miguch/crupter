#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

use clap::{App, AppSettings, Arg, SubCommand};
mod args;
mod config;
mod handlers;
mod hashes;
mod utils;

use std::io::Write;

fn main() -> Result<(), failure::Error> {
    let config = config::init();
    let mut app = App::new("Crupter")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Miguel Chan <i@miguch.com>")
        .about("An easy tool for file hash and encryption");
    let cpus = num_cpus::get().to_string();

    for action in &config.checksums {
        app = app.subcommand(
            SubCommand::with_name(action.name.as_str())
                .about(action.help_msg.as_str())
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .value_name("FILE")
                        .help("The file(s) to be encrypted")
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("parallels")
                        .short("j")
                        .long("parallels")
                        .default_value(&cpus)
                        .help("Number of parallel jobs."),
                ),
        )
    }
    for action in &config.ciphers {
        app = app.subcommand(
            SubCommand::with_name(action.name.as_str())
                .about(action.help_msg.as_str())
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(
                    Arg::with_name("passphrase")
                        .short("p")
                        .long("passphrase")
                        .value_name("STRING")
                        .help("The passphrase for file encryption")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .value_name("FILE")
                        .help("The file(s) to be encrypted")
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("PATH")
                        .help("The output name of encrypted file. \nSupported fields: [index, filename]\n")
                        .takes_value(true)
                        .default_value("crypted/{{index}}.ci"),
                )
                .arg(
                    Arg::with_name("decrypt")
                        .short("d")
                        .long("decrypt")
                        .help("Specify to decrypt file."),
                )
                .arg(
                    Arg::with_name("parallels")
                        .short("j")
                        .long("parallels")
                        .default_value(&cpus)
                        .help("Number of parallel jobs."),
                ),
        )
    }

    let matches = app.get_matches();

    // iterate through all actions to find what to do
    for action in config.checksums.iter().chain(config.ciphers.iter()) {
        if let Some(matches) = matches.subcommand_matches(&action.name) {
            if let Err(error) = (action.handler)(matches) {
                writeln!(&mut std::io::stderr(), "{}", error).unwrap()
            }
            break;
        }
    }
    Ok(())
}

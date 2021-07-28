#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate hex_literal;

use clap::{App, crate_version, AppSettings, Arg, SubCommand};
mod args;
mod ciphers;
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
        .about("An easy tool for file hash and encryption")
        .version(crate_version!());
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
                )
                .arg(
                    Arg::with_name("silent")
                        .short("s")
                        .long("silent")
                        .help("specify to hide progress bar"),
                ),
        )
    }
    for action in &config.ciphers {
        app = app.subcommand(
            SubCommand::with_name(action.name.as_str())
                .about(action.help_msg.as_str())
                .setting(AppSettings::ArgRequiredElseHelp)
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
                        .help("The output name of en(de)crypted file. \nSupported fields: [index, filename]\n[ default:\n\tencrypt: crypted/{{index}}.ci\n\tdecrypt: output/{{filename}}\n]")
                        .takes_value(true)
                        .default_value("")
                        .hide_default_value(true)
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
                )
                .arg(
                    Arg::with_name("silent")
                        .short("s")
                        .long("silent")
                        .help("specify to hide progress bar"),
                )
                .arg(
                    Arg::with_name("list-name")
                        .short("l")
                        .long("list-name")
                        .help("only show file name when decrypting")
                )
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

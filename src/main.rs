#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings, Arg, SubCommand};
mod args;
mod config;
mod handlers;
mod utils;

fn main() {
    let config = config::init();
    let mut app = App::new("Crupter")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Miguel Chan <i@miguch.com>")
        .about("A easy tool for file hash and encryption");

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
                ),
        )
    }
    for action in &config.ciphers {
        app = app.subcommand(
            SubCommand::with_name(action.name.as_str())
                .about(action.help_msg.as_str())
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
                ),
        )
    }

    let matches = app.get_matches();

    // iterate through all actions to find what to do
    for action in &config.checksums {
        if let Some(matches) = matches.subcommand_matches(&action.name) {
            (action.handler)(matches);
            break;
        }
    }
    for action in &config.ciphers {
        if let Some(matches) = matches.subcommand_matches(&action.name) {
            (action.handler)(matches);
            break;
        }
    }
}

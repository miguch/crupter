use crate::utils::mustache;
use clap::ArgMatches;
use colored::*;
use std::io::Write;

#[derive(Debug)]
pub struct HashArgs {
    filenames: Vec<String>,
}

impl From<&ArgMatches<'_>> for HashArgs {
    fn from(matches: &ArgMatches) -> Self {
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(String::from).collect()
        } else {
            vec![]
        };
        Self { filenames }
    }
}

#[derive(Debug)]
pub struct CipherArgs {
    passphrase: String,
    filenames: Vec<String>,
    output_template: mustache::MustacheExp,
    decrypt: bool,
}

impl From<&ArgMatches<'_>> for CipherArgs {
    fn from(matches: &ArgMatches) -> Self {
        let passphrase = matches.value_of("passphrase").map(String::from).unwrap();
        let template_str = matches.value_of("output").unwrap();
        let output_template =
            mustache::compile_mustache(template_str, false).unwrap_or_else(|err| {
                writeln!(
                    &mut std::io::stderr(),
                    "fail to compile output template: {}",
                    format!("{}", err).red()
                )
                .unwrap();
                std::process::exit(1);
            });
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(String::from).collect()
        } else {
            vec![]
        };
        Self {
            passphrase,
            filenames,
            output_template,
            decrypt: matches.is_present("decrypt"),
        }
    }
}

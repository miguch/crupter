use crate::utils::mustache;
use clap::ArgMatches;
use std::convert::TryFrom;
use std::io::Write;

#[derive(Debug)]
pub struct HashArgs {
    pub filenames: Vec<String>,
    pub parallels: u32,
}

impl TryFrom<&ArgMatches<'_>> for HashArgs {
    type Error = failure::Error;
    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(String::from).collect()
        } else {
            vec![]
        };
        let parallels = matches.value_of("parallels").unwrap().parse::<u32>()?;
        Ok(Self {
            filenames,
            parallels,
        })
    }
}

#[derive(Debug)]
pub struct CipherArgs {
    pub passphrase: String,
    pub filenames: Vec<String>,
    pub output_template: mustache::MustacheExp,
    pub decrypt: bool,
    pub parallels: u32,
}

impl TryFrom<&ArgMatches<'_>> for CipherArgs {
    type Error = failure::Error;
    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let passphrase = matches.value_of("passphrase").map(String::from).unwrap();
        let template_str = matches.value_of("output").unwrap();
        let output_template = mustache::compile_mustache(template_str, false)?;
        let parallels = matches.value_of("parallels").unwrap().parse::<u32>()?;
        mustache::render(&output_template, std::collections::HashMap::new())?;
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(String::from).collect()
        } else {
            vec![]
        };
        Ok(Self {
            passphrase,
            filenames,
            output_template,
            decrypt: matches.is_present("decrypt"),
            parallels,
        })
    }
}

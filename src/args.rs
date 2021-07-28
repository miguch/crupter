use crate::utils::errors::ReadError;
use crate::utils::mustache;
use clap::ArgMatches;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HashArgs {
    pub filenames: Vec<PathBuf>,
    pub parallels: u32,
    pub silent: bool,
}

impl TryFrom<&ArgMatches<'_>> for HashArgs {
    type Error = failure::Error;
    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(PathBuf::from).collect()
        } else {
            vec![]
        };
        let parallels = matches.value_of("parallels").unwrap().parse::<u32>()?;
        Ok(Self {
            filenames,
            parallels,
            silent: matches.is_present("silent"),
        })
    }
}

#[derive(Debug)]
pub struct CipherArgs {
    pub passphrase: String,
    pub filenames: Vec<PathBuf>,
    pub output_template: mustache::MustacheExp,
    pub decrypt: bool,
    pub parallels: u32,
    pub silent: bool,
    pub list_name: bool,
}

impl TryFrom<&ArgMatches<'_>> for CipherArgs {
    type Error = failure::Error;
    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let passphrase = matches.value_of("passphrase").map(String::from).unwrap();
        let decrypt = matches.is_present("decrypt");
        let mut template_str = matches
            .value_of("output")
            .map(|tpl| {
                // if empty set default value
                match tpl {
                    "" if decrypt => "output/{{filename}}",
                    "" if !decrypt => "crypted/{{index}}.ci",
                    _ => tpl,
                }
            })
            .unwrap()
            .to_owned();
        if &template_str[0..2] == "~/" {
            template_str.replace_range(
                0..1,
                dirs::home_dir()
                    .ok_or(ReadError::NoHomeDir)?
                    .to_str()
                    .unwrap(),
            );
        }
        // Check output directory, create if not exist
        let output_dir = std::path::Path::new(&template_str)
            .parent()
            .unwrap_or(std::path::Path::new(&template_str));
        if output_dir.as_os_str() != "" && !output_dir.exists() && !matches.is_present("list-name") {
            std::fs::create_dir(output_dir).map_err(|err| ReadError::CreateDirError {
                err,
                dir: output_dir.to_string_lossy().to_string(),
            })?;
            println!("Created directory: {}", output_dir.display());
        }
        let output_template = mustache::compile_mustache(&template_str, false)?;
        let parallels = matches.value_of("parallels").unwrap().parse::<u32>()?;
        let filenames = if let Some(files) = matches.values_of("file") {
            files.map(PathBuf::from).collect()
        } else {
            std::fs::read_dir(".")
                .map_err(|err| ReadError::ReadDirError {
                    err,
                    dir: ".".to_owned(),
                })?
                .map(|entry| Ok(entry?.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()?
        };
        let list_name = matches.is_present("list-name");
        Ok(Self {
            passphrase,
            filenames,
            output_template,
            decrypt,
            parallels,
            silent: matches.is_present("silent") || (decrypt && list_name),
            list_name,
        })
    }
}

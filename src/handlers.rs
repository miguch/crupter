use crate::utils::mustache;
use clap::ArgMatches;

struct HashArgs {
    filenames: Vec<String>,
}

struct CipherArgs {
    passphrase: String,
    filenames: Vec<String>,
    output_template: Vec<mustache::Token>
}

pub fn sha1_handler(matches: &ArgMatches) {
    println!("SHA1");
}
pub fn sha2_handler(matches: &ArgMatches) {
    println!("SHA2");
}
pub fn sha3_handler(matches: &ArgMatches) {
    println!("SHA1");
}
pub fn md5_handler(matches: &ArgMatches) {
    println!("md5");
}
pub fn aes_handler(matches: &ArgMatches) {
    println!("{:?}", matches)
}

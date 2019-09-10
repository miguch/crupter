use crate::args::{CipherArgs, HashArgs};
use clap::ArgMatches;

pub fn sha1_handler(matches: &ArgMatches) {
    let args = HashArgs::from(matches);
    println!("{:?}", args);
    println!("SHA1");
}
pub fn sha2_handler(matches: &ArgMatches) {
    let args = HashArgs::from(matches);
    println!("SHA2");
}
pub fn sha3_handler(matches: &ArgMatches) {
    let args = HashArgs::from(matches);
    println!("SHA1");
}
pub fn md5_handler(matches: &ArgMatches) {
    let args = HashArgs::from(matches);
    println!("md5");
}
pub fn aes_handler(matches: &ArgMatches) {
    let args = CipherArgs::from(matches);
    println!("{:?}", args);
    println!("{:?}", matches)
}

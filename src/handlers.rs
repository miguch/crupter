use crate::args::{CipherArgs, HashArgs};
use crate::hashes::hasher::Hasher;
use crate::utils::parallel::set_num_threads;
use clap::ArgMatches;
use digest::Digest;
use rayon::prelude::*;
use std::path::Path;

use std::convert::TryFrom;

fn hash_handler<D: Digest>(matches: &ArgMatches) -> Result<(), failure::Error> {
    let args = HashArgs::try_from(matches)?;
    set_num_threads(args.parallels as usize);
    let output_size = D::output_size();
    if args.filenames.is_empty() {
        let hasher = D::new();
        hasher.from_reader(std::io::stdin().lock())?;
    } else {
        let multi_bar = indicatif::MultiProgress::new();
        let pbs: Vec<indicatif::ProgressBar> = (0..args.filenames.len())
            .map(|_| multi_bar.add(indicatif::ProgressBar::new(0)))
            .collect();
        let multi_bar_thread = std::thread::spawn(move || {
            // use a single thread to handle multi progress bar render
            multi_bar.join().unwrap();
        });
        let hash_results: Vec<_> = args
            .filenames
            .par_iter()
            .zip(pbs)
            .map(|(file, pb)| {
                let hasher = D::new();
                (file, hasher.from_file(Path::new(&file), Some(pb)))
            })
            .collect();

        for (file, result) in hash_results {
            match result {
                Err(err) => println!("[{}] error: {}", file, err),
                Ok(hash) => println!("[{}]: {:?}", file, hash),
            }
        }
        multi_bar_thread.join().unwrap()
    }
    Ok(())
}

pub fn sha1_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha1::Sha1>(matches)
}
pub fn sha2_256_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha2::Sha256>(matches)
}
pub fn sha2_224_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha2::Sha224>(matches)
}
pub fn sha2_384_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha2::Sha384>(matches)
}
pub fn sha2_512_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha2::Sha512>(matches)
}
pub fn sha3_256_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha3::Sha3_256>(matches)
}
pub fn sha3_224_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha3::Sha3_224>(matches)
}
pub fn sha3_384_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha3::Sha3_384>(matches)
}
pub fn sha3_512_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<sha3::Sha3_512>(matches)
}
pub fn md5_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    hash_handler::<md5::Md5>(matches)
}

pub fn aes_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    let args = CipherArgs::try_from(matches)?;
    Ok(())
}

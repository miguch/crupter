use crate::args::{CipherArgs, HashArgs};
use crate::ciphers::{cipher::CrupterCipher, passphrase};
use crate::hashes::hasher::Hasher;
use crate::utils::mustache;
use crate::utils::parallel::set_num_threads;
use crate::utils::progress_read::{prepare_multi_bar, ProgressRead};
use clap::ArgMatches;
use digest::Digest;
use rayon::prelude::*;
use std::collections::HashMap;
use stream_cipher::{NewStreamCipher, SyncStreamCipher};

use std::convert::TryFrom;

fn hash_handler<D: Digest>(matches: &ArgMatches) -> Result<(), failure::Error> {
    let args = HashArgs::try_from(matches)?;
    set_num_threads(args.parallels as usize);
    if args.filenames.is_empty() {
        let hasher = D::new();
        let result = hasher.from_reader(std::io::stdin().lock())?;
        result
            .as_slice()
            .iter()
            .for_each(|byte| print!("{:x}", byte));
        println!("");
    } else {
        let (pbs, multi_bar_thread) = prepare_multi_bar(args.filenames.len());

        let hash_outputs: Vec<_> = args
            .filenames
            .par_iter()
            .zip(pbs)
            .map(|(file, pb)| {
                let hasher = D::new();
                (
                    file,
                    match ProgressRead::from_file_path(file, pb) {
                        Ok(progress_file) => hasher.from_reader(progress_file),
                        Err(err) => Err(err),
                    },
                )
            })
            .collect();

        for (file, result) in hash_outputs {
            match result {
                Err(err) => println!("[{:?}] error: {}", file, err),
                Ok(hash) => {
                    print!("[{:?}] = ", file);
                    hash.as_slice().iter().for_each(|byte| print!("{:x}", byte));
                    println!("");
                }
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

use generic_array::typenum::Unsigned;
fn cipher_handler<C: NewStreamCipher + SyncStreamCipher>(
    matches: &ArgMatches,
) -> Result<(), failure::Error> {
    let args = CipherArgs::try_from(matches)?;
    set_num_threads(args.parallels as usize);
    let key_len = C::KeySize::to_usize();
    let iv_len = C::NonceSize::to_usize();
    let mut key_iv = vec![0; key_len + iv_len];
    passphrase::generate(&args.passphrase, &mut key_iv);
    let key = &key_iv[..key_len];
    let iv = &key_iv[key_len..];

    let (pbs, multi_bar_thread) = prepare_multi_bar(args.filenames.len());

    let encrypt_results: Vec<_> = args
        .filenames
        .par_iter()
        .enumerate()
        .zip(pbs)
        .map(|((index, file), pb)| {
            let cipher = C::new_var(&key, &iv).unwrap();
            (
                file,
                match ProgressRead::from_file_path(file, pb) {
                    Ok(progress_file) => {
                        if args.decrypt {
                            cipher.decrypt_file(
                                progress_file,
                                &args.output_template,
                                &args.passphrase,
                            )
                        } else {
                            let filename = file.file_name().unwrap().to_string_lossy().into_owned();
                            let render_info = {
                                let mut map = HashMap::with_capacity(2);
                                map.insert("index", index.to_string());
                                map.insert("filename", filename.clone());
                                map
                            };
                            match mustache::render(&args.output_template, &render_info) {
                                Err(err) => Err(err),
                                Ok(out_name) => cipher.encrypt_file(
                                    progress_file,
                                    &filename,
                                    out_name,
                                    &args.passphrase,
                                ),
                            }
                        }
                    }
                    Err(err) => Err(err),
                },
            )
        })
        .collect();

    for (file, result) in encrypt_results {
        match result {
            Err(err) => println!("[{:?}] error: {}", file, err),
            Ok(out_file) => {
                println!("[{:?}] => {:?} ", file, out_file);
            }
        }
    }
    multi_bar_thread.join().unwrap();
    Ok(())
}

pub fn aes_128_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    cipher_handler::<aesni::Aes128Ctr>(matches)
}

pub fn aes_192_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    cipher_handler::<aesni::Aes192Ctr>(matches)
}

pub fn aes_256_handler(matches: &ArgMatches) -> Result<(), failure::Error> {
    cipher_handler::<aesni::Aes256Ctr>(matches)
}

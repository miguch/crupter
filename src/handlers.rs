use crate::args::{CipherArgs, HashArgs};
use crate::ciphers::{cipher::CrupterCipher, passphrase};
use crate::hashes::hasher::Hasher;
use crate::utils::errors::ReadError;
use crate::utils::mustache;
use crate::utils::parallel::set_num_threads;
use crate::utils::progress_read::{prepare_multi_bar, ProgressRead};
use anyhow::Result;
use ctr::cipher::{KeyIvInit, StreamCipher};
use ctr::cipher::typenum::Unsigned;
use digest::Digest;
use rayon::prelude::*;
use std::collections::HashMap;

fn resolve_parallels(parallels: u32) -> u32 {
    if parallels == 0 {
        num_cpus::get() as u32
    } else {
        parallels
    }
}

fn hash_handler<D: Digest>(args: HashArgs) -> Result<()> {
    let parallels = resolve_parallels(args.parallels);
    set_num_threads(parallels as usize);
    if args.filenames.is_empty() {
        let hasher = D::new();
        let result = hasher.from_reader(std::io::stdin().lock())?;
        for byte in result.as_slice() {
            print!("{:02x}", byte);
        }
        println!();
    } else {
        let (pbs, multi_bar_thread) = prepare_multi_bar(args.filenames.len(), args.silent);

        let hash_outputs: Vec<_> = args
            .filenames
            .par_iter()
            .zip(pbs)
            .map(|(file, pb)| {
                let hasher = D::new();
                (
                    file,
                    match ProgressRead::from_file_path(file, pb, args.silent) {
                        Ok(progress_file) => hasher.from_reader(progress_file),
                        Err(err) => Err(err),
                    },
                )
            })
            .collect();

        multi_bar_thread.join().unwrap();
        for (file, result) in hash_outputs {
            match result {
                Err(err) => println!("[{:?}] error: {}", file, err),
                Ok(hash) => {
                    print!("[{:?}] = ", file);
                    for byte in hash.as_slice() {
                        print!("{:02x}", byte);
                    }
                    println!();
                }
            }
        }
    }
    Ok(())
}

pub fn sha1_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha1::Sha1>(args)
}
pub fn sha2_256_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha2::Sha256>(args)
}
pub fn sha2_224_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha2::Sha224>(args)
}
pub fn sha2_384_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha2::Sha384>(args)
}
pub fn sha2_512_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha2::Sha512>(args)
}
pub fn sha3_256_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha3::Sha3_256>(args)
}
pub fn sha3_224_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha3::Sha3_224>(args)
}
pub fn sha3_384_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha3::Sha3_384>(args)
}
pub fn sha3_512_handler(args: HashArgs) -> Result<()> {
    hash_handler::<sha3::Sha3_512>(args)
}
pub fn md5_handler(args: HashArgs) -> Result<()> {
    hash_handler::<md5::Md5>(args)
}

fn cipher_handler<C: KeyIvInit + StreamCipher>(args: CipherArgs) -> Result<()> {
    let parallels = resolve_parallels(args.parallels);
    set_num_threads(parallels as usize);
    let key_len = C::KeySize::to_usize();
    let iv_len = C::IvSize::to_usize();
    let mut key_iv = vec![0; key_len + iv_len];
    passphrase::generate(&args.passphrase, &mut key_iv);
    let key = &key_iv[..key_len];
    let iv = &key_iv[key_len..];

    let output_template = (|| -> Result<mustache::MustacheExp> {
        let mut template_str = (&args.output).to_string();
        if template_str.is_empty() {
            template_str = if args.decrypt {
                "output/{{filename}}".to_string()
            } else {
                "crypted/{{index}}.ci".to_string()
            };
        }
        if template_str.starts_with("~/") {
            template_str.replace_range(
                0..1,
                dirs::home_dir()
                    .ok_or(ReadError::NoHomeDir)?
                    .to_str()
                    .unwrap(),
            );
        }
        let output_dir = std::path::Path::new(&template_str)
            .parent()
            .unwrap_or(std::path::Path::new(&template_str));
        if output_dir.as_os_str() != "" && !output_dir.exists() && !args.list_name {
            std::fs::create_dir(output_dir).map_err(|err| ReadError::CreateDirError {
                err,
                dir: output_dir.to_string_lossy().to_string(),
            })?;
            println!("Created directory: {}", output_dir.display());
        }
        mustache::compile_mustache(&template_str, false)
    })()?;

    let filenames = if args.filenames.is_empty() {
        std::fs::read_dir(".")
            .map_err(|err| ReadError::ReadDirError {
                err,
                dir: ".".to_owned(),
            })?
            .map(|entry| Ok(entry?.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?
    } else {
        args.filenames.clone()
    };

    let silent = args.silent || (args.decrypt && args.list_name);

    let (pbs, multi_bar_thread) = prepare_multi_bar(filenames.len(), silent);

    let count = std::sync::Mutex::new(0);

    let encrypt_results: Vec<_> = filenames
        .par_iter()
        .enumerate()
        .zip(pbs)
        .map(|((index, file), pb)| {
            let cipher = C::new_from_slices(key, iv).unwrap();
            (
                file,
                match ProgressRead::from_file_path(file, pb, silent) {
                    Ok(progress_file) => {
                        if silent {
                            let mut count = count.lock().unwrap();
                            *count += 1;
                            println!("[{}] {:?}", count, file);
                        }
                        if args.decrypt {
                            cipher.decrypt_file(
                                progress_file,
                                &output_template,
                                &args.passphrase,
                                args.list_name,
                            )
                        } else {
                            let filename =
                                file.file_name().unwrap().to_string_lossy().into_owned();
                            let render_info = {
                                let mut map = HashMap::with_capacity(2);
                                map.insert("index", index.to_string());
                                map.insert("filename", filename.clone());
                                map
                            };
                            match mustache::render(&output_template, &render_info) {
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

    multi_bar_thread.join().unwrap();
    for (file, result) in encrypt_results {
        match result {
            Err(err) => println!("[{:?}] error: {}", file, err),
            Ok(out_file) => {
                println!("[{:?}] => {:?} ", file, out_file);
            }
        }
    }
    Ok(())
}

pub fn aes_128_handler(args: CipherArgs) -> Result<()> {
    cipher_handler::<crate::ciphers::Aes128Ctr>(args)
}

pub fn aes_192_handler(args: CipherArgs) -> Result<()> {
    cipher_handler::<crate::ciphers::Aes192Ctr>(args)
}

pub fn aes_256_handler(args: CipherArgs) -> Result<()> {
    cipher_handler::<crate::ciphers::Aes256Ctr>(args)
}

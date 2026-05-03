use ctr::cipher::typenum::Unsigned;
use digest::Digest;
use hex_literal::hex;
use rand::{thread_rng, RngCore};
use scrypt::{scrypt, Params};

use ctr::cipher::KeyIvInit;
use ctr::cipher::generic_array::GenericArray;

pub const HASHED_PWD_LENGTH: usize = 64;

pub fn validate(received: &str, stored: &[u8]) -> bool {
    let hashed_pwd = get_hashed_pwd(received);
    hashed_pwd.iter().zip(stored.iter()).all(|(a, b)| a == b)
}

pub fn generate_bytes(output: &mut [u8]) {
    let mut rng = thread_rng();
    rng.fill_bytes(output);
}

pub fn generate_var<S: KeyIvInit>(
) -> (GenericArray<u8, S::KeySize>, GenericArray<u8, S::IvSize>) {
    let mut rng = thread_rng();
    let key_len = S::KeySize::to_usize();
    let iv_len = S::IvSize::to_usize();
    let mut key_iv = vec![0; key_len + iv_len];
    rng.fill_bytes(&mut key_iv);
    (
        GenericArray::clone_from_slice(&key_iv[..key_len]),
        GenericArray::clone_from_slice(&key_iv[..iv_len]),
    )
}

pub fn get_hashed_pwd(password: &str) -> Vec<u8> {
    let first_hashed = sha2::Sha512::digest(password.as_bytes());
    let second_hashed = sha3::Sha3_512::digest(&first_hashed);
    second_hashed.as_slice().to_vec()
}

pub fn generate(password: &str, output: &mut [u8]) {
    let params = Params::new(15, 8, 1, output.len()).unwrap();
    let salt = hex!("12 a3 c9 12 90 89 65 aa be fc");

    scrypt(password.as_bytes(), &salt, &params, output).unwrap();
}

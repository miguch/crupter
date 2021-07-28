use digest::Digest;
use generic_array_cipher::typenum::Unsigned;
use generic_array_cipher::GenericArray;
use rand::{thread_rng, Rng};
use scrypt::{scrypt, ScryptParams};
use ofb::cipher::{NewCipher};

pub const HASHED_PWD_LENGTH: usize = 64;

pub fn validate(received: &str, stored: &[u8]) -> bool {
    let hashed_pwd = get_hashed_pwd(received);
    hashed_pwd.iter().zip(stored.iter()).all(|(a, b)| a == b)
}

pub fn generate_bytes(output: &mut [u8]) {
    let mut rng = thread_rng();
    output.iter_mut().for_each(|byte| *byte = rng.gen::<u8>());
}

pub fn generate_var<S: NewCipher>(
) -> (GenericArray<u8, S::KeySize>, GenericArray<u8, S::NonceSize>) {
    let _rng = thread_rng();
    let key_len = S::KeySize::to_usize();
    let iv_len = S::NonceSize::to_usize();
    let mut key_iv = vec![0; key_len + iv_len];
    generate_bytes(&mut key_iv);
    (
        GenericArray::clone_from_slice(&key_iv[..key_len]),
        GenericArray::clone_from_slice(&key_iv[..iv_len]),
    )
}

pub fn get_hashed_pwd(password: &str) -> Vec<u8> {
    let mut hasher = sha2::Sha512::new();
    hasher.input(password.as_bytes());
    let first_hashed = hasher.result();
    let mut hasher = sha3::Sha3_512::new();
    hasher.input(&first_hashed);
    hasher.result().as_slice().to_vec()
}

pub fn generate(password: &str, output: &mut [u8]) {
    let params = ScryptParams::new(15, 8, 1).unwrap();
    let salt = hex!("12 a3 c9 12 90 89 65 aa be fc");

    scrypt(password.as_bytes(), &salt, &params, output).unwrap();
}

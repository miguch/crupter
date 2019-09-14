use digest::Digest;
use scrypt::{scrypt, ScryptParams};

pub const HASHED_PWD_LENGTH: usize = 64;

pub fn validate(received: &str, stored: &[u8]) -> bool {
    let hashed_pwd = get_hashed_pwd(received);
    hashed_pwd.iter().zip(stored.iter()).all(|(a, b)| a == b)
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

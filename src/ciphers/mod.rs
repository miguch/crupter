pub mod cipher;
pub mod passphrase;

use aes::{Aes128, Aes192, Aes256};

pub type Aes128Ctr = ctr::Ctr128BE<Aes128>;
pub type Aes192Ctr = ctr::Ctr128BE<Aes192>;
pub type Aes256Ctr = ctr::Ctr128BE<Aes256>;

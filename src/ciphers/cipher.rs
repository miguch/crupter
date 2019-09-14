use crate::ciphers::passphrase;
use crate::utils::mustache;
use mustache::MustacheExp;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use stream_cipher::{NewStreamCipher, SyncStreamCipher};

pub trait CrupterCipher {
    fn apply<R: Read, W: Write>(&mut self, reader: R, writer: W) -> Result<u64, failure::Error>;
    fn encrypt_file<P: AsRef<Path>, R: Read>(
        self,
        file_reader: R,
        out_name: P,
        password: &str,
    ) -> Result<PathBuf, failure::Error>;
    fn decrypt_file<R: Read>(
        self,
        file_reader: R,
        out_name_template: &MustacheExp,
        password: &str,
    ) -> Result<PathBuf, failure::Error>;
}

impl<S: NewStreamCipher + SyncStreamCipher> CrupterCipher for S {
    fn apply<R: Read, W: Write>(&mut self, reader: R, writer: W) -> Result<u64, failure::Error> {
        unimplemented!()
    }
    fn encrypt_file<P: AsRef<Path>, R: Read>(
        mut self,
        file_reader: R,
        out_name: P,
        password: &str,
    ) -> Result<PathBuf, failure::Error> {
        let mut hashed_pwd = passphrase::get_hashed_pwd(password);
        self.apply_keystream(&mut hashed_pwd);

        let out_file = std::fs::File::create(out_name)?;
        let out_buf = std::io::BufWriter::new(out_file);
        unimplemented!()
    }
    fn decrypt_file<R: Read>(
        mut self,
        file_reader: R,
        out_name_template: &MustacheExp,
        password: &str,
    ) -> Result<PathBuf, failure::Error> {
        unimplemented!()
    }
}

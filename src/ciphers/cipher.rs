use crate::ciphers::passphrase;
use crate::utils::mustache;
use mustache::MustacheExp;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use stream_cipher::{NewStreamCipher, SyncStreamCipher};

pub trait CrupterCipher {
    fn apply<R: Read, W: Write>(&mut self, reader: R, writer: W) -> Result<usize, failure::Error>;
    fn encrypt_file<P: AsRef<Path>, R: Read>(
        self,
        file_reader: R,
        file_name: &str,
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
    fn apply<R: Read, W: Write>(&mut self, mut reader: R, mut writer: W) -> Result<usize, failure::Error> {
        // Use a 10mb buffer
        let mut buffer = vec![0; 10485760];
        let mut total_bytes = 0;
        loop {
            let read_bytes = reader.read(&mut buffer)?;
            total_bytes += read_bytes;
            if read_bytes == 0 {
                break;
            }
            self.apply_keystream(&mut buffer[..read_bytes]);
            writer.write(&buffer[..read_bytes])?;
        }
        Ok(total_bytes)
    }
    fn encrypt_file<P: AsRef<Path>, R: Read>(
        mut self,
        file_reader: R,
        file_name: &str,
        out_name: P,
        password: &str,
    ) -> Result<PathBuf, failure::Error> {
        let mut hashed_pwd = passphrase::get_hashed_pwd(password);
        self.apply_keystream(&mut hashed_pwd);

        let out_file = std::fs::File::create(&out_name)?;
        let mut out_buf = std::io::BufWriter::new(out_file);
        out_buf.write(&hashed_pwd)?;

        let (mut key, mut iv) = passphrase::generate_var::<Self>();
        // Use a random cipher to encrypt file
        let mut file_cipher = Self::new_var(&key, &iv).unwrap();
        self.apply_keystream(&mut key);
        self.apply_keystream(&mut iv);
        // Save random cipher key and iv
        out_buf.write(&key)?;
        out_buf.write(&iv)?;

        let mut file_header = Vec::<u8>::with_capacity(file_name.len() + 4);
        // 4 bytes to indicate file name length 
        file_header.extend(&(file_name.len() as u32).to_le_bytes());
        file_header.extend(file_name.as_bytes());

        file_cipher.apply_keystream(&mut file_header);
        out_buf.write(&file_header)?;

        // Encrypt the whole file
        file_cipher.apply(file_reader, out_buf)?;

        Ok(PathBuf::from(out_name.as_ref()))
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

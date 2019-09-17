use crate::ciphers::passphrase;
use crate::utils::errors::CipherError;
use crate::utils::mustache;
use generic_array::typenum::Unsigned;
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
    fn apply<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
    ) -> Result<usize, failure::Error> {
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
            writer.write_all(&buffer[..read_bytes])?;
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
        out_buf.write_all(&hashed_pwd)?;

        let (mut key, mut iv) = passphrase::generate_var::<Self>();
        // Use a random cipher to encrypt file
        let mut file_cipher = Self::new_var(&key, &iv).unwrap();
        self.apply_keystream(&mut key);
        self.apply_keystream(&mut iv);
        // Save random cipher key and iv
        out_buf.write_all(&key)?;
        out_buf.write_all(&iv)?;

        let mut file_header = Vec::<u8>::with_capacity(file_name.len() + 4);
        // 4 bytes to indicate file name length
        file_header.extend(&(file_name.len() as u32).to_le_bytes());
        file_header.extend(file_name.as_bytes());

        file_cipher.apply_keystream(&mut file_header);
        out_buf.write_all(&file_header)?;

        // Encrypt the whole file
        file_cipher.apply(file_reader, out_buf)?;

        Ok(PathBuf::from(out_name.as_ref()))
    }
    fn decrypt_file<R: Read>(
        mut self,
        mut file_reader: R,
        out_name_template: &MustacheExp,
        password: &str,
    ) -> Result<PathBuf, failure::Error> {
        // Validate with passphrase hash
        let mut hashed_pwd = vec![0; passphrase::HASHED_PWD_LENGTH];
        file_reader.read_exact(&mut hashed_pwd)?;
        self.apply_keystream(&mut hashed_pwd);
        if !passphrase::validate(password, &hashed_pwd) {
            Err(CipherError::PasswordError)?;
        }
        //Get encrypted key & iv
        let key_len = S::KeySize::to_usize();
        let iv_len = S::NonceSize::to_usize();
        let mut key_iv = vec![0; key_len + iv_len];
        file_reader.read_exact(&mut key_iv)?;
        self.apply_keystream(&mut key_iv);
        let mut file_cipher = Self::new_var(&key_iv[..key_len], &key_iv[key_len..]).unwrap();
        // Get filename
        let mut filename_len_buf = [0; 4];
        file_reader.read_exact(&mut filename_len_buf)?;
        file_cipher.apply_keystream(&mut filename_len_buf);
        let filename_len = u32::from_le_bytes(filename_len_buf) as usize;
        let mut filename_buf = vec![0; filename_len];
        file_reader.read_exact(&mut filename_buf)?;
        file_cipher.apply_keystream(&mut filename_buf);
        let render_map = {
            let mut map = std::collections::HashMap::new();
            map.insert("filename", std::str::from_utf8(&filename_buf)?.to_owned());
            map
        };
        let output_name = mustache::render(out_name_template, &render_map)?;
        let output_file = std::fs::File::create(&output_name)?;
        let output_buf = std::io::BufWriter::new(output_file);
        file_cipher.apply(file_reader, output_buf)?;
        Ok(PathBuf::from(output_name))
    }
}

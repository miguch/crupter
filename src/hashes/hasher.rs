use crate::utils::{errors::ReadError, progress_read::ProgressRead};
use digest::Digest;
use generic_array::GenericArray;
use std::io::Read;
use std::path::Path;

pub trait Hasher {
    type OutputArray;
    /// Produce a hash result from a reader(stdin, file, ...)
    fn from_reader<R: Read>(self, reader: R) -> Result<Self::OutputArray, failure::Error>;
    fn from_file<P: AsRef<Path>>(
        self,
        path: P,
        pb: Option<indicatif::ProgressBar>,
    ) -> Result<Self::OutputArray, failure::Error>
    where
        Self: std::marker::Sized,
    {
        let file = std::fs::File::open(&path).map_err(|err| ReadError::OpenFileError { err })?;
        let filename = path.as_ref().file_name().unwrap();
        let meta = file
            .metadata()
            .map_err(|err| ReadError::MetadataError { err })?;
        if !meta.is_file() {
            Err(ReadError::NotFileError)?;
        }
        let file_size = meta.len();
        let buf_file = std::io::BufReader::new(file);
        if let Some(pb) = pb {
            self.from_reader(ProgressRead::new(
                buf_file,
                file_size,
                filename.to_str().unwrap(),
                pb,
            ))
        } else {
            self.from_reader(buf_file)
        }
    }
}

impl<D: Digest> Hasher for D {
    type OutputArray = GenericArray<u8, D::OutputSize>;

    fn from_reader<R: Read>(mut self, mut reader: R) -> Result<Self::OutputArray, failure::Error> {
        // Use a 10mb buffer
        let mut buffer = vec![0; 10485760];
        loop {
            let read_bytes = reader.read(&mut buffer)?;
            if read_bytes == 0 {
                break;
            } else {
                self.input(&buffer[..read_bytes]);
            }
        }
        Ok(self.result())
    }
}

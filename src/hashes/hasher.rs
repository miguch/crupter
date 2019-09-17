
use digest::Digest;
use generic_array::GenericArray;
use std::io::Read;


pub trait Hasher {
    type OutputArray;
    /// Produce a hash result from a reader(stdin, file, ...)
    fn from_reader<R: Read>(self, reader: R) -> Result<Self::OutputArray, failure::Error>;
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

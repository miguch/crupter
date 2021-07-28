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

pub trait PDHasher {
    /// Consume buffer
    fn consume(&mut self, buf: &[u8]);
    /// output result
    fn produce(self: Box<Self>) -> Result<String, failure::Error>;
}

impl<D: Digest> PDHasher for D {
    fn consume(&mut self, buf: &[u8]) {
        self.input(buf)
    }
    fn produce(self: Box<Self>) -> Result<String, failure::Error> {
        use std::fmt::Write;
        let hash = self.result();
        let mut result = String::new();
        for byte in hash.as_slice() {
            write!(&mut result, "{:x}", byte)?;
        }
        Ok(result)
    }
}

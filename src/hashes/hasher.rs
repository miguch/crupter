use digest::Digest;
use std::io::Read;

pub trait Hasher {
    type OutputArray;
    fn from_reader<R: Read>(self, reader: R) -> Result<Self::OutputArray, anyhow::Error>;
}

impl<D: Digest> Hasher for D {
    type OutputArray = digest::Output<D>;

    fn from_reader<R: Read>(mut self, mut reader: R) -> Result<Self::OutputArray, anyhow::Error> {
        let mut buffer = vec![0; 10485760];
        loop {
            let read_bytes = reader.read(&mut buffer)?;
            if read_bytes == 0 {
                break;
            } else {
                Digest::update(&mut self, &buffer[..read_bytes]);
            }
        }
        Ok(self.finalize())
    }
}

pub trait PDHasher {
    fn consume(&mut self, buf: &[u8]);
    fn produce(self: Box<Self>) -> Result<String, anyhow::Error>;
}

impl<D: Digest> PDHasher for D {
    fn consume(&mut self, buf: &[u8]) {
        Digest::update(self, buf)
    }
    fn produce(self: Box<Self>) -> Result<String, anyhow::Error> {
        use std::fmt::Write;
        let hash = self.finalize();
        let mut result = String::new();
        for byte in hash.as_slice() {
            write!(&mut result, "{:x}", byte)?;
        }
        Ok(result)
    }
}

use crupter::utils::errors::ReadError;
use std::io::Read;

pub type FileRead = StatusRead<std::io::BufReader<std::fs::File>>;

pub struct FinishedInfo {
    pub name: String,
    pub result: String,
    pub bytes: u64,
}
pub enum ReadItem {
    File(FileRead),
    Done(FinishedInfo),
    Failed(String, anyhow::Error),
}

pub enum ReadState {
    Going,
    Finished,
    Error(std::io::ErrorKind, String),
}

pub struct StatusRead<R: Read> {
    pub inner: R,
    pub bytes: u64,
    pub current: u64,
    pub state: ReadState,
    pub name: String,
}

impl ReadItem {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        match FileRead::from_file_path(&path) {
            Ok(read) => Self::File(read),
            Err(err) => Self::Failed(
                path.as_ref()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                err,
            ),
        }
    }
}

impl FileRead {
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, anyhow::Error> {
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
        Ok(StatusRead {
            inner: buf_file,
            bytes: file_size,
            current: 0,
            state: ReadState::Going,
            name: filename.to_string_lossy().to_string(),
        })
    }
}

impl<R: Read> Read for StatusRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        if let ReadState::Error(kind, err) = &self.state {
            Err(std::io::Error::new(*kind, err.as_str()))?;
        }
        let read_size = self.inner.read(buf).map_err(|err| {
            self.state = ReadState::Error(err.kind(), err.to_string());
            err
        })?;
        self.current += read_size as u64;
        if self.current >= self.bytes {
            self.state = ReadState::Finished;
        }
        Ok(read_size)
    }
}

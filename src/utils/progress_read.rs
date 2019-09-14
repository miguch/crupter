use crate::utils::errors::ReadError;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

pub fn prepare_multi_bar(bars: usize) -> (Vec<ProgressBar>, std::thread::JoinHandle<()>) {
    let multi_bar = indicatif::MultiProgress::new();
    let pbs: Vec<indicatif::ProgressBar> = (0..bars)
        .map(|_| multi_bar.add(indicatif::ProgressBar::new(0)))
        .collect();
    let multi_bar_thread = std::thread::spawn(move || {
        // use a single thread to handle multi progress bar render
        multi_bar.join().unwrap();
    });
    (pbs, multi_bar_thread)
}

/// A Read with a progress bar
pub struct ProgressRead<R: Read> {
    /// Underlying Read Object
    inner: R,
    /// Total bytes to read
    bytes: u64,
    /// The progress bar
    bar: ProgressBar,
    /// Indicate whether the read has finished,
    /// progress bar can only finish once
    finished: bool,
}

impl<R: Read> ProgressRead<R> {
    pub fn new(inner: R, bytes: u64, name: &str, pb: ProgressBar) -> Self {
        pb.set_prefix(name);
        pb.set_length(bytes);
        let template = name.to_owned()
            + " [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg}";
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&template)
                .progress_chars("#>-"),
        );
        Self {
            inner,
            bytes,
            bar: pb,
            finished: false,
        }
    }
}

impl ProgressRead<std::io::BufReader<std::fs::File>> {
    pub fn from_file_path<P: AsRef<std::path::Path>>(
        path: P,
        pb: ProgressBar,
    ) -> Result<Self, failure::Error> {
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
        Ok(ProgressRead::new(
            buf_file,
            file_size,
            filename.to_str().unwrap(),
            pb,
        ))
    }
}

impl<R: Read> Read for ProgressRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let read_size = self.inner.read(buf).map_err(|err| {
            self.bar.finish_at_current_pos();
            err
        })?;
        let new_position = self.bar.position() + read_size as u64;
        if !self.finished {
            if new_position >= self.bytes {
                self.finished = true;
                self.bar.finish_with_message("Done.");
            } else {
                self.bar.set_position(new_position);
            }
        }
        Ok(read_size)
    }
}

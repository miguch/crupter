use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

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
                self.bar.finish();
            } else {
                self.bar.set_position(new_position);
            }
        }
        Ok(read_size)
    }
}

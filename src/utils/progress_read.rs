use crate::utils::errors::ReadError;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

pub fn prepare_multi_bar(
    bars: usize,
    silent: bool,
) -> (Vec<Option<ProgressBar>>, std::thread::JoinHandle<()>) {
    let mut multi_bar = if silent {
        None
    } else {
        Some(indicatif::MultiProgress::new())
    };
    let pbs = (0..bars)
        .map(|_| {
            if silent {
                None
            } else {
                Some(
                    multi_bar
                        .as_mut()
                        .unwrap()
                        .add(indicatif::ProgressBar::new(0)),
                )
            }
        })
        .collect();
    let multi_bar_thread = std::thread::spawn(move || {
        // use a single thread to handle multi progress bar render
        if let Some(bars) = multi_bar {
            bars.join().unwrap();
        }
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
    bar: Option<ProgressBar>,
    /// Indicate whether the read has finished,
    /// progress bar can only finish once
    finished: bool,
    // no progress bar if true
    silent: bool,
    // read position
    position: u64,
}

impl<R: Read> ProgressRead<R> {
    pub fn new(inner: R, bytes: u64, name: &str, mut pb: Option<ProgressBar>, silent: bool) -> Self {
        if let Some(bar) = &mut pb {
            bar.set_prefix(name);
            bar.set_length(bytes);
            let template = name.to_owned()
                + " [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg}";
            bar.set_style(
                ProgressStyle::default_bar()
                    .template(&template)
                    .progress_chars("#>-"),
            );
        }
        Self {
            inner,
            bytes,
            bar: pb,
            finished: false,
            silent,
            position: 0,
        }
    }
}

impl ProgressRead<std::io::BufReader<std::fs::File>> {
    pub fn from_file_path<P: AsRef<std::path::Path>>(
        path: P,
        pb: Option<ProgressBar>,
        silent: bool,
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
            silent,
        ))
    }
}

impl<R: Read> Read for ProgressRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let read_size = self.inner.read(buf).map_err(|err| {
            if let Some(bar) = &self.bar {
                bar.finish_at_current_pos();
            }
            err
        })?;
        let new_position = self.position + read_size as u64;
        if !self.finished && !self.silent {
            if new_position >= self.bytes {
                self.finished = true;
                if let Some(bar) = &self.bar {
                    bar.finish_with_message("Done.");
                }
            } else {
                if let Some(bar) = &self.bar {
                    bar.set_position(new_position);
                }
            }
        }
        self.position = new_position;
        Ok(read_size)
    }
}

impl<R: Read> Drop for ProgressRead<R> {
    fn drop(&mut self) {
        if !self.finished {
            if let Some(bar) = &self.bar {
                // error happened else where, clear progress bar here
                bar.finish_and_clear();
            }
        }
    }
}

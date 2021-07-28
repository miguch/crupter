use crupter::hashes::*;
use hasher::PDHasher;
use libc::c_char;
use rayon::prelude::*;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::io::Read;

use crate::status_info::StatusInfo;
use crate::status_read::{FinishedInfo, ReadItem};

lazy_static! {
    static ref HASH_METHODS: RwLock<Vec<(&'static str, bool)>> = {
        RwLock::new(
            vec![
                "md5", "sha1", "sha224", "sha256", "sha384", "sha512", "sha3-224", "sha3-256",
                "sha3-384", "sha3-512",
            ]
            .iter()
            .map(|name| (*name, true))
            .collect(),
        )
    };
    static ref HASH_FILES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
    static ref HASH_PROGRESS: Mutex<Vec<Arc<RwLock<ReadItem>>>> = Mutex::new(Vec::new());
}

struct Hashes {
    names: Vec<&'static str>,
    hashers: Vec<Box<dyn PDHasher>>,
}

impl Hashes {
    pub fn new() -> Self {
        use digest::Digest;
        let mut hashers = Vec::<Box<dyn PDHasher>>::new();
        let mut names = Vec::new();
        let methods = HASH_METHODS.read().unwrap();
        for (name, active) in &*methods {
            if *active {
                names.push(*name);
                hashers.push(match *name {
                    "md5" => Box::new(Md5::new()),
                    "sha1" => Box::new(Sha1::new()),
                    "sha224" => Box::new(Sha224::new()),
                    "sha256" => Box::new(Sha256::new()),
                    "sha384" => Box::new(Sha384::new()),
                    "sha512" => Box::new(Sha512::new()),
                    "sha3-224" => Box::new(Sha3_224::new()),
                    "sha3-256" => Box::new(Sha3_256::new()),
                    "sha3-384" => Box::new(Sha3_384::new()),
                    "sha3-512" => Box::new(Sha3_512::new()),
                    _ => panic!("unexpected hash method"),
                });
            }
        }
        Self {
            names,
            hashers,
        }
    }

    pub fn consume(&mut self, reader: Arc<RwLock<ReadItem>>) -> Result<(), failure::Error> {
        // Use a 10mb buffer
        let mut buffer = vec![0; 10485760];
        loop {
            let mut read_guard = reader.write().unwrap();
            if let &mut ReadItem::File(ref mut reader) = &mut *read_guard {
                let read_bytes = reader.read(&mut buffer)?;
                if read_bytes == 0 {
                    break;
                } else {
                    self.hashers.iter_mut().for_each(|val| {
                        val.consume(&buffer[..read_bytes]);
                    })
                }
            }
        }
        Ok(())
    }

    pub fn produce(self) -> String {
        self.names
            .iter()
            .zip(self.hashers.into_iter())
            .map(|(key, val)| {
                format!(
                    "{}: {}",
                    key,
                    val.produce().unwrap_or_else(|e| e.to_string())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[no_mangle]
pub extern "C" fn add_hash_file(path_raw: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!path_raw.is_null());
        CStr::from_ptr(path_raw)
    };
    let mut files = HASH_FILES.lock().unwrap();
    files.push(PathBuf::from(c_str.to_string_lossy().into_owned()));
    files.len() as u32
}

#[no_mangle]
pub extern "C" fn clean_hash_files() {
    let mut files = HASH_FILES.lock().unwrap();
    files.clear();
}

#[no_mangle]
pub extern "C" fn remove_hash_file(index: u32) {
    let mut files = HASH_FILES.lock().unwrap();
    files.remove(index as usize);
}

#[no_mangle]
pub extern "C" fn get_hash_files_num() -> u32 {
    let files = HASH_FILES.lock().unwrap();
    files.len() as u32
}

#[no_mangle]
pub extern "C" fn run_hash_session() {
    // non-blocking
    std::thread::spawn(move || {
        let files = HASH_FILES.lock().unwrap();
        let progresses: Vec<_> = files
            .par_iter()
            .map(|file_path| Arc::new(RwLock::new(ReadItem::new(file_path))))
            .collect();
        {
            let mut global_prog = HASH_PROGRESS.lock().unwrap();
            *global_prog = progresses.clone();
        }
        progresses.par_iter().for_each(|reader| {
            let read_guard = reader.read().unwrap();
            if let ReadItem::File(read) = &*read_guard {
                let name = read.name.clone();
                let bytes = read.bytes;
                drop(read_guard);
                let mut hashes = Hashes::new();
                if let Err(err) = hashes.consume(reader.clone()) {
                    let mut read_guard = reader.write().unwrap();
                    *read_guard = ReadItem::Failed(name, err);
                } else {
                    let mut read_guard = reader.write().unwrap();
                    let result = hashes.produce();
                    println!("{}", result);
                    *read_guard = ReadItem::Done(FinishedInfo {
                        name,
                        bytes,
                        result,
                    });
                }
            }
        });
    });
}

#[no_mangle]
pub extern "C" fn get_hash_file(index: u32) -> *mut c_char {
    let files = HASH_FILES.lock().unwrap();
    let val = CString::new(files[index as usize].to_string_lossy().into_owned()).unwrap();
    let ptr = val.into_raw();
    ptr
}

#[no_mangle]
pub extern "C" fn get_hash_progress(index: u32) -> StatusInfo {
    let progresses = HASH_PROGRESS.lock().unwrap();
    let item = progresses[index as usize].read().unwrap();
    match &*item {
        ReadItem::File(reader) => StatusInfo {
            status: 0,
            name: CString::new(reader.name.as_str()).unwrap().into_raw(),
            result: CString::new("").unwrap().into_raw(),
            current: reader.current,
            bytes: reader.bytes,
        },
        ReadItem::Done(info) => StatusInfo {
            status: 1,
            name: CString::new(info.name.as_str()).unwrap().into_raw(),
            result: CString::new(info.result.as_str()).unwrap().into_raw(),
            current: info.bytes,
            bytes: info.bytes,
        },
        ReadItem::Failed(name, err) => StatusInfo {
            status: 2,
            name: CString::new(name.as_str()).unwrap().into_raw(),
            result: CString::new(err.to_string()).unwrap().into_raw(),
            current: 0,
            bytes: 0,
        },
    }
}

#[no_mangle]
pub extern "C" fn hash_running_count() -> u32 {
    let progresses = HASH_PROGRESS.lock().unwrap();
    progresses.len() as u32
}

#[no_mangle]
pub extern "C" fn print_info(info: StatusInfo) {
    println!("{:?}", info);
}

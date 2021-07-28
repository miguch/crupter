#[macro_use]
extern crate lazy_static;
use libc::c_char;
use status_read::ReadItem;
use std::ffi::CString;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

mod hash;
mod status_info;
mod status_read;

pub use hash::*;
use status_info::StatusInfo;

lazy_static! {
    static ref CIPHER_FILES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
    static ref CIPHER_PROGRESS: Mutex<Vec<Arc<RwLock<ReadItem>>>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub unsafe extern "C" fn free_status_info(item: StatusInfo) {
    free_rust_cstr(item.name);
    free_rust_cstr(item.result);
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_cstr(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    let _ = CString::from_raw(ptr);
}

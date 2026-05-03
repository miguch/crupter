use libc::c_char;
use std::ffi::CString;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

mod hash;
mod status_info;
mod status_read;

use status_info::StatusInfo;
use status_read::ReadItem;

static CIPHER_FILES: LazyLock<Mutex<Vec<PathBuf>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static CIPHER_PROGRESS: LazyLock<Mutex<Vec<Arc<RwLock<ReadItem>>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

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

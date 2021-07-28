use libc::c_char;
#[repr(C)]
#[derive(Debug)]
pub struct StatusInfo {
    // 0 - Going, 1 - Done, 2 - Failed
    pub status: i32,
    pub name: *mut c_char,
    pub result: *mut c_char,
    pub current: u64,
    pub bytes: u64,
}

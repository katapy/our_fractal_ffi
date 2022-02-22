
use std::ffi::CStr;
use libc::c_char;
use std::path::PathBuf;

extern crate our_fractal_core;
use our_fractal_core::{Manager};

#[no_mangle]
pub extern "C" fn make_manager(path: *const c_char, table_name: *const c_char, data_name: *const c_char) -> *mut Manager {
    unsafe{
        let path: &PathBuf = &PathBuf::from(CStr::from_ptr(path).to_str().unwrap().to_string());
        let table_name: String = CStr::from_ptr(table_name).to_str().unwrap().to_string();
        let data_name: String = CStr::from_ptr(data_name).to_str().unwrap().to_string();
        let s = Box::new(Manager::new(path, table_name, data_name));
        Box::into_raw(s)
    }
}

#[no_mangle]
pub extern "C" fn destroy_manager(p: *mut Manager) {
    unsafe { Box::from_raw(p) };
}

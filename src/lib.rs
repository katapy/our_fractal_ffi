
use std::ffi::{CStr, CString};
use libc::c_char;
use std::path::PathBuf;

extern crate our_fractal_core;
use our_fractal_core::{Manager, Type, Definition};
pub mod ffi_def;
use crate::ffi_def::ffi_def::{FFIDef};

/// Create new manager.
/// * `path` - Data folder path.
/// * `table_name` - Data table name.
/// * `data_name` - Data name.
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

/// Destriy manager.
/// * `p` - Destroyed manager.
#[no_mangle]
pub extern "C" fn destroy_manager(p: *mut Manager) {
    unsafe { Box::from_raw(p) };
}

/// Create new manager.
/// * `path` - Data folder path.
/// * `table_name` - Data table name.
/// * `data_name` - Data name.
#[no_mangle]
pub extern "C" fn make_definition(manager: *mut Manager, tag: u32) -> *mut Definition {
    unsafe{
        let s = Box::new((*manager).get_def(&tag).unwrap().clone());
        Box::into_raw(s)
    }
}

/// Destriy manager.
/// * `p` - Destroyed manager.
#[no_mangle]
pub extern "C" fn destroy_definition(p: *mut Definition) {
    unsafe { Box::from_raw(p) };
}

/// Create new manager.
/// * `manager` - Manager pointer.
/// * `tag` - Definition tag.
/// * `name` - Definition name.
/// * `data_type` - Definition Data type.
/// * `is_multiple` - Definition is multiple.
#[no_mangle]
pub extern "C" fn add_definition(
    manager: *mut Manager, tag: u32, name: *const c_char, data_type: Type, is_multiple: bool) -> bool
{
    unsafe{
        let name = CStr::from_ptr(name).to_str().unwrap().to_string();
        match (*manager).add_def(tag, name, data_type, is_multiple){
            Ok(_) => true, 
            Err(_) => false
        }
    }
}

/// Destriy manager.
/// * `manager` - Manager pointer.
#[no_mangle]
pub extern "C" fn get_def_list(manager: *mut Manager) -> CString {
    let mut tag_list = String::new();
    unsafe {
        for i in (*manager).get_def_tag_list() {
            tag_list += format!("{:>08x} ", i).as_str();
        }
    };
    CString::new(tag_list.trim()).unwrap()
}

#[no_mangle]
pub extern "C" fn get_def(definition: *mut Definition) -> FFIDef{
    unsafe {
        let def = &(*definition).clone(); 
        FFIDef::new(def.tag, def.data_type, def.is_multiple)
    }
}

/// Get definition name
/// * `definition` - Definition ptr.
#[no_mangle]
pub extern "C" fn get_def_name(definition: *mut Definition) -> CString{
    CString::new(unsafe { 
        (*definition).clone().get_name().clone()
    }).unwrap()
}

/// Get definition explanation
/// * `definition` - Definition ptr.
#[no_mangle]
pub extern "C" fn get_def_exp(definition: *mut Definition) -> CString{
    CString::new(unsafe { 
        (*definition).clone().get_explanation().clone()
    }).unwrap()
}

/// Set definition name
/// * `definition` - Definition ptr.
/// * `tag` - Definition tag.
#[no_mangle]
pub extern "C" fn set_def_exp(definition: *mut Definition, exp: *const c_char) {
    unsafe {
        let exp: String = CStr::from_ptr(exp).to_str().unwrap().to_string();
        (*definition).set_explanation(exp)
    }
}

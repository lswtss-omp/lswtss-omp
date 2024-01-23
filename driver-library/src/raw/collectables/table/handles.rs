use std::ffi::c_void;

pub static mut RAW_COLLECTABLE_TABLE_HANDLES: Vec<*mut c_void> = Vec::new();

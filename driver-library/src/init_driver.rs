use std::ffi::c_void;
use std::ptr::null_mut;

use winapi::um::libloaderapi::GetModuleHandleW;

use crate::*;

pub fn init_driver()
{
    init_driver_debug_console_client();

    let process_exe_module_handle = unsafe { GetModuleHandleW(null_mut()) as *mut c_void };

    register_raw(process_exe_module_handle);

    test_api::init();

    register_mods();

    load_mods();

    link_custom_resources();

    std::thread::spawn(js_engine_thread_main);
}

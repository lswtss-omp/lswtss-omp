use std::ffi::c_int;
use std::ffi::c_void;

/// Hints:
/// - registered as `nttFile::FileDevicePC::vftable[24]`
/// - calls other function that calls `CreateFileA`, `CreateFileW` and other Windows API functions
const RAW_NTT_FILE_FILE_DEVICE_PC_VTABLE_24_METHOD_OFFSET: isize = 0x3402b40;

pub type RawNttFileFileDevicePcVtable24Method = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
    unknown_param_4: c_int,
) -> *mut c_void;

pub static mut RAW_NTT_FILE_FILE_DEVICE_PC_VTABLE_24_METHOD_PTR_OPTION: Option<
    RawNttFileFileDevicePcVtable24Method,
> = None;

pub fn register_raw_ntt_file_file_device_pc_vtable_24_method_ptr(
    process_exe_module_handle: *mut c_void
)
{
    unsafe {
        RAW_NTT_FILE_FILE_DEVICE_PC_VTABLE_24_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_NTT_FILE_FILE_DEVICE_PC_VTABLE_24_METHOD_OFFSET),
            ),
        );
    }
}

use std::ffi::c_char;
use std::ffi::c_void;

/// Hints:
/// - references `nttFile::FileDevicePC::vftable`
/// - calls `eh_vector_constructor_iterator`
const RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_OFFSET: isize = 0x3401910;

pub type RawNttFileFileDevicePcConstructor = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    unknown_param_2: *mut c_char,
    unknown_param_3: *mut c_void,
) -> *mut c_void;

pub static mut RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_PTR_OPTION: Option<
    RawNttFileFileDevicePcConstructor,
> = None;

pub fn register_raw_ntt_file_file_device_pc_constructor_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_OFFSET),
            ),
        );
    }
}

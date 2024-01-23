use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::*;

declare_raw_fn_hook!(
    RawNttFileFileDevicePcConstructor,
    RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_PTR_OPTION,
    RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_HOOK_TRAMPOLINE,
    raw_ntt_file_file_device_pc_constructor_hook_detour,
    register_raw_ntt_file_file_device_pc_constructor_hook,
);

pub unsafe extern "stdcall" fn raw_ntt_file_file_device_pc_constructor_hook_detour(
    handle: *mut c_void,
    unknown_param_2: *mut c_char,
    unknown_param_3: *mut c_void,
) -> *mut c_void
{
    let unknown_param_2_as_str = CStr::from_ptr(unknown_param_2)
        .to_str()
        .unwrap();

    if unknown_param_2_as_str == "host:" {
        RAW_NTT_FILE_HOST_FILE_DEVICE_PC_HANDLE_OPTION = Some(handle);

        log::info!(
            "RAW_NTT_FILE_HOST_FILE_DEVICE_PC_HANDLE: {:?}",
            RAW_NTT_FILE_HOST_FILE_DEVICE_PC_HANDLE_OPTION.unwrap(),
        );
    }

    return RAW_NTT_FILE_FILE_DEVICE_PC_CONSTRUCTOR_HOOK_TRAMPOLINE(
        handle,
        unknown_param_2,
        unknown_param_3,
    );
}

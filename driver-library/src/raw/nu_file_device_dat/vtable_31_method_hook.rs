use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::*;

declare_raw_fn_hook!(
    RawNuFileDeviceDatVtable31Method,
    RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_PTR_OPTION,
    RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_HOOK_TRAMPOLINE,
    raw_nu_file_device_dat_vtable_31_method_hook_detour,
    register_raw_nu_file_device_dat_vtable_31_method_hook,
);

pub unsafe extern "stdcall" fn raw_nu_file_device_dat_vtable_31_method_hook_detour(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
) -> c_char
{
    let unknown_param_2_as_str = CStr::from_ptr(*(unknown_param_2 as *mut *mut c_char))
        .to_str()
        .unwrap();

    if let Some(raw_nu_file_device_dat_handle) = RAW_NU_FILE_DEVICE_DAT_HANDLE_OPTION {
        if raw_nu_file_device_dat_handle == handle {
            let resource_canon_path = get_resource_canon_path(unknown_param_2_as_str);

            if fetch_is_disk_resource_registered(&resource_canon_path) {
                return 1;
            }
        }
    }

    return RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_HOOK_TRAMPOLINE(
        handle,
        unknown_param_2,
        unknown_param_3,
    );
}

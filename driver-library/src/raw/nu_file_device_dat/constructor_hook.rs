use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawNuFileDeviceDatConstructor,
    RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_PTR_OPTION,
    RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_HOOK_TRAMPOLINE,
    raw_nu_file_device_dat_constructor_hook_detour,
    register_raw_nu_file_device_dat_constructor_hook,
);

pub unsafe extern "stdcall" fn raw_nu_file_device_dat_constructor_hook_detour(
    handle: *mut c_void
) -> *mut c_void
{
    RAW_NU_FILE_DEVICE_DAT_HANDLE_OPTION = Some(handle);

    log::info!(
        "RAW_NU_FILE_DEVICE_DAT_HANDLE: {:?}",
        RAW_NU_FILE_DEVICE_DAT_HANDLE_OPTION.unwrap(),
    );

    return RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_HOOK_TRAMPOLINE(handle);
}

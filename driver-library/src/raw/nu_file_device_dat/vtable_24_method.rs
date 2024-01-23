use std::ffi::c_int;
use std::ffi::c_void;

/// Hints:
/// - registered as `NuFileDeviceDat::vftable[24]`
const RAW_NU_FILE_DEVICE_DAT_VTABLE_24_METHOD_OFFSET: isize = 0x33f7570;

pub type RawNuFileDeviceDatVtable24Method = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
    unknown_param_4: c_int,
) -> *mut c_void;

pub static mut RAW_NU_FILE_DEVICE_DAT_VTABLE_24_METHOD_PTR_OPTION: Option<
    RawNuFileDeviceDatVtable24Method,
> = None;

pub fn register_raw_nu_file_device_dat_vtable_24_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_NU_FILE_DEVICE_DAT_VTABLE_24_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_NU_FILE_DEVICE_DAT_VTABLE_24_METHOD_OFFSET),
            ),
        );
    }
}

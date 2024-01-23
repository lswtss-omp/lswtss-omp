use std::ffi::c_char;
use std::ffi::c_void;

/// Hints:
/// - registered as `NuFileDeviceDat::vftable[31]`
const RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_OFFSET: isize = 0x33f7990;

pub type RawNuFileDeviceDatVtable31Method = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
) -> c_char;

pub static mut RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_PTR_OPTION: Option<
    RawNuFileDeviceDatVtable31Method,
> = None;

pub fn register_raw_nu_file_device_dat_vtable_31_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_NU_FILE_DEVICE_DAT_VTABLE_31_METHOD_OFFSET),
            ),
        );
    }
}

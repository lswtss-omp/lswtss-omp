use std::ffi::c_void;

/// Hints:
/// - references `NuFileDeviceDat::vftable`
/// - calls `eh_vector_constructor_iterator`
const RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_OFFSET: isize = 0x33f6720;

pub type RawNuFileDeviceDatConstructor =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub static mut RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_PTR_OPTION: Option<
    RawNuFileDeviceDatConstructor,
> = None;

pub fn register_raw_nu_file_device_dat_constructor_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_NU_FILE_DEVICE_DAT_CONSTRUCTOR_OFFSET),
            ),
        );
    }
}

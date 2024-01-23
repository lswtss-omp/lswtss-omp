use std::ffi::c_void;

/// Hints:
/// - registered to AngelScript Engine
const RAW_API_COMPONENT_ENABLE_METHOD_OFFSET: isize = 0x2DD9810;

pub type RawApiComponentEnableMethod = unsafe extern "stdcall" fn(handle: *mut c_void);

pub static mut RAW_API_COMPONENT_ENABLE_METHOD_PTR_OPTION: Option<RawApiComponentEnableMethod> =
    None;

pub fn register_raw_api_component_enable_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_API_COMPONENT_ENABLE_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_API_COMPONENT_ENABLE_METHOD_OFFSET),
            ),
        );
    }
}

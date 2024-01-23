use std::ffi::c_void;

/// Hints:
/// - registered to AngelScript Engine
const RAW_API_COMPONENT_GET_PARENT_METHOD_OFFSET: isize = 0xC7F60;

pub type RawApiComponentGetParentMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub static mut RAW_API_COMPONENT_GET_PARENT_METHOD_PTR_OPTION: Option<
    RawApiComponentGetParentMethod,
> = None;

pub fn register_raw_api_component_get_parent_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_API_COMPONENT_GET_PARENT_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_API_COMPONENT_GET_PARENT_METHOD_OFFSET),
            ),
        );
    }
}

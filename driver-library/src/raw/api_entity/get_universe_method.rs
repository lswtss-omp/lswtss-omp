use std::ffi::c_void;

/// Hints:
/// - registered to AngelScript Engine
const RAW_API_ENTITY_GET_UNIVERSE_METHOD_OFFSET: isize = 0x31A39C;

pub type RawApiEntityGetUniverseMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub static mut RAW_API_ENTITY_GET_UNIVERSE_METHOD_PTR_OPTION: Option<
    RawApiEntityGetUniverseMethod,
> = None;

pub fn register_raw_api_entity_get_universe_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_API_ENTITY_GET_UNIVERSE_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_API_ENTITY_GET_UNIVERSE_METHOD_OFFSET),
            ),
        );
    }
}

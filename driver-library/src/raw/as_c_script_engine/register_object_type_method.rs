use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - registered as `asCScriptEngine::vftable[19]`
const RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_OFFSET: isize = 0x2d59b50;

pub type RawAsCScriptEngineRegisterObjectTypeMethod = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    name: *mut c_char,
    byte_size: c_int,
    flags: c_int,
) -> c_uint;

pub static mut RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_PTR_OPTION: Option<
    RawAsCScriptEngineRegisterObjectTypeMethod,
> = None;

pub fn register_raw_as_c_script_engine_register_object_type_method_ptr(
    process_exe_module_handle: *mut c_void
)
{
    unsafe {
        RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_OFFSET),
            ),
        );
    }
}

use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_ENGINE_GET_ENUM_COUNT_METHOD_VTABLE_INDEX: isize = 35;

pub type RawAsIScriptEngineGetEnumCountMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_uint;

pub unsafe fn raw_as_i_script_engine_get_enum_count_method(handle: *mut c_void) -> c_uint
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptEngineGetEnumCountMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_ENGINE_GET_ENUM_COUNT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

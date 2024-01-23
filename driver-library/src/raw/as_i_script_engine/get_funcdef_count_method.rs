use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_ENGINE_GET_FUNCDEF_COUNT_METHOD_VTABLE_INDEX: isize = 38;

pub type RawAsIScriptEngineGetFuncdefCountMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_uint;

pub unsafe fn raw_as_i_script_engine_get_funcdef_count_method(handle: *mut c_void) -> c_uint
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptEngineGetFuncdefCountMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_ENGINE_GET_FUNCDEF_COUNT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

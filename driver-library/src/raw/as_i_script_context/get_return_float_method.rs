use std::ffi::c_float;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_FLOAT_METHOD_VTABLE_INDEX: isize = 27;

pub type RawAsIScriptContextGetReturnFloatMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_float;

pub unsafe fn raw_as_i_script_context_get_return_float_method(handle: *mut c_void) -> c_float
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnFloatMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_FLOAT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

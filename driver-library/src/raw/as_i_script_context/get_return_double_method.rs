use std::ffi::c_double;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_DOUBLE_METHOD_VTABLE_INDEX: isize = 28;

pub type RawAsIScriptContextGetReturnDoubleMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_double;

pub unsafe fn raw_as_i_script_context_get_return_double_method(handle: *mut c_void) -> c_double
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnDoubleMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_DOUBLE_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

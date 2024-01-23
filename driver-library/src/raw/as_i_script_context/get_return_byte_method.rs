use std::ffi::c_uchar;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_BYTE_METHOD_VTABLE_INDEX: isize = 23;

pub type RawAsIScriptContextGetReturnByteMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_uchar;

pub unsafe fn raw_as_i_script_context_get_return_byte_method(handle: *mut c_void) -> c_uchar
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnByteMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_BYTE_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

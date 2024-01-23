use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_D_WORD_METHOD_VTABLE_INDEX: isize = 25;

pub type RawAsIScriptContextGetReturnDWordMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_uint;

pub unsafe fn raw_as_i_script_context_get_return_d_word_method(handle: *mut c_void) -> c_uint
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnDWordMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_D_WORD_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

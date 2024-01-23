use std::ffi::c_ushort;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_WORD_METHOD_VTABLE_INDEX: isize = 24;

pub type RawAsIScriptContextGetReturnWordMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_ushort;

pub unsafe fn raw_as_i_script_context_get_return_word_method(handle: *mut c_void) -> c_ushort
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnWordMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_WORD_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

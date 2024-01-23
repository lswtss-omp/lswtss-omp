use std::ffi::c_ulonglong;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_Q_WORD_METHOD_VTABLE_INDEX: isize = 26;

pub type RawAsIScriptContextGetReturnQWordMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_ulonglong;

pub unsafe fn raw_as_i_script_context_get_return_q_word_method(handle: *mut c_void) -> c_ulonglong
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnQWordMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_Q_WORD_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

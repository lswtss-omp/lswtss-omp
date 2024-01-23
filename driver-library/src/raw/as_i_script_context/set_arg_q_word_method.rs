use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_Q_WORD_METHOD_VTABLE_INDEX: isize = 16;

pub type RawAsIScriptContextSetArgQWordMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, arg: c_uint, value: c_ulong) -> c_int;

pub unsafe fn raw_as_i_script_context_set_arg_q_word_method(
    handle: *mut c_void,
    arg: c_uint,
    value: c_ulong,
) -> c_int
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextSetArgQWordMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_Q_WORD_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle, arg, value,
    );
}

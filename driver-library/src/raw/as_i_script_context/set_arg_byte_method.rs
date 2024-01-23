use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_BYTE_METHOD_VTABLE_INDEX: isize = 13;

pub type RawAsIScriptContextSetArgByteMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, arg: c_uint, value: c_uchar) -> c_int;

pub unsafe fn raw_as_i_script_context_set_arg_byte_method(
    handle: *mut c_void,
    arg: c_uint,
    value: c_uchar,
) -> c_int
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextSetArgByteMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_BYTE_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle, arg, value,
    );
}

use std::ffi::c_float;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_FLOAT_METHOD_VTABLE_INDEX: isize = 17;

pub type RawAsIScriptContextSetArgFloatMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, arg: c_uint, value: c_float) -> c_int;

pub unsafe fn raw_as_i_script_context_set_arg_float_method(
    handle: *mut c_void,
    arg: c_uint,
    value: c_float,
) -> c_int
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextSetArgFloatMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_SET_ARG_FLOAT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle, arg, value,
    );
}

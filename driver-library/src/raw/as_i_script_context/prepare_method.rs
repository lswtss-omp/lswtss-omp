use std::ffi::c_int;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_PREPARE_METHOD_VTABLE_INDEX: isize = 3;

pub type RawAsIScriptContextPrepareMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, func: *mut c_void) -> c_int;

pub unsafe fn raw_as_i_script_context_prepare_method(
    handle: *mut c_void,
    func: *mut c_void,
) -> c_int
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextPrepareMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_PREPARE_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle, func,
    );
}

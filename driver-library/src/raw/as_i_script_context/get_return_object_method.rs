use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_OBJECT_METHOD_VTABLE_INDEX: isize = 30;

pub type RawAsIScriptContextGetReturnObjectMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub unsafe fn raw_as_i_script_context_get_return_object_method(handle: *mut c_void) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnObjectMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_OBJECT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

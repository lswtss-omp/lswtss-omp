use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_ADDRESS_METHOD_VTABLE_INDEX: isize = 29;

pub type RawAsIScriptContextGetReturnAddressMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub unsafe fn raw_as_i_script_context_get_return_address_method(handle: *mut c_void)
    -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptContextGetReturnAddressMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_CONTEXT_GET_RETURN_ADDRESS_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

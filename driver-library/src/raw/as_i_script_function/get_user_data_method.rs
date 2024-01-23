use std::ffi::c_longlong;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_FUNCTION_GET_USER_DATA_METHOD_VTABLE_INDEX: isize = 38;

pub type RawAsIScriptFunctionGetUserDataMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, user_data_type: c_longlong) -> *mut c_void;

pub unsafe fn raw_as_i_script_function_get_user_data_method(
    handle: *mut c_void,
    user_data_type: c_longlong,
) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptFunctionGetUserDataMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_FUNCTION_GET_USER_DATA_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle,
        user_data_type,
    );
}

use std::ffi::c_char;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_TYPE_INFO_GET_NAMESPACE_METHOD_VTABLE_INDEX: isize = 7;

pub type RawAsITypeInfoGetNamespaceMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_char;

pub unsafe fn raw_as_i_type_info_get_namespace_method(handle: *mut c_void) -> *mut c_char
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsITypeInfoGetNamespaceMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_TYPE_INFO_GET_NAMESPACE_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

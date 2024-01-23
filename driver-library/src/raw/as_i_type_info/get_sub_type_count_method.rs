use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_TYPE_INFO_GET_SUB_TYPE_COUNT_METHOD_VTABLE_INDEX: isize = 15;

pub type RawAsITypeInfoGetSubTypeCountMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> c_uint;

pub unsafe fn raw_as_i_type_info_get_sub_type_count_method(handle: *mut c_void) -> c_uint
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsITypeInfoGetSubTypeCountMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_TYPE_INFO_GET_SUB_TYPE_COUNT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

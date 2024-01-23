use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_TYPE_INFO_GET_METHOD_BY_INDEX_METHOD_VTABLE_INDEX: isize = 23;

pub type RawAsITypeInfoGetMethodByIndexMethod = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    index: c_uint,
    get_virtual: c_char,
) -> *mut c_void;

pub unsafe fn raw_as_i_type_info_get_method_by_index_method(
    handle: *mut c_void,
    index: c_uint,
    get_virtual: c_char,
) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsITypeInfoGetMethodByIndexMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_TYPE_INFO_GET_METHOD_BY_INDEX_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle,
        index,
        get_virtual,
    );
}

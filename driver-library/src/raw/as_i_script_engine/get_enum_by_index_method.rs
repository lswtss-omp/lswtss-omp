use std::ffi::c_uint;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_ENGINE_GET_ENUM_BY_INDEX_METHOD_VTABLE_INDEX: isize = 36;

pub type RawAsIScriptEngineGetEnumByIndexMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, index: c_uint) -> *mut c_void;

pub unsafe fn raw_as_i_script_engine_get_enum_by_index_method(
    handle: *mut c_void,
    index: c_uint,
) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptEngineGetEnumByIndexMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_ENGINE_GET_ENUM_BY_INDEX_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle, index,
    );
}

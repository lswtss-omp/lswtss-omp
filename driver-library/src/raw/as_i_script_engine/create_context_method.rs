use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_ENGINE_CREATE_CONTEXT_METHOD_VTABLE_INDEX: isize = 60;

pub type RawAsIScriptEngineCreateContextMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void) -> *mut c_void;

pub unsafe fn raw_as_i_script_engine_create_context_method(handle: *mut c_void) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptEngineCreateContextMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_ENGINE_CREATE_CONTEXT_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(handle);
}

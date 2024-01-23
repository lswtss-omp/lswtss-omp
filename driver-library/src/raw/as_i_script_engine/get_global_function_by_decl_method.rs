use std::ffi::c_char;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_ENGINE_GET_GLOBAL_FUNCTION_BY_DECL_METHOD_VTABLE_INDEX: isize = 13;

pub type RawAsIScriptEngineGetGlobalFunctionByDeclMethod =
    unsafe extern "stdcall" fn(handle: *mut c_void, declaration: *const c_char) -> *mut c_void;

pub unsafe fn raw_as_i_script_engine_get_global_function_by_decl_method(
    handle: *mut c_void,
    declaration: *const c_char,
) -> *mut c_void
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptEngineGetGlobalFunctionByDeclMethod = std::mem::transmute(
        *(vtable.byte_offset(
            (RAW_AS_I_SCRIPT_ENGINE_GET_GLOBAL_FUNCTION_BY_DECL_METHOD_VTABLE_INDEX) * 8,
        )),
    );

    return method(
        handle,
        declaration,
    );
}

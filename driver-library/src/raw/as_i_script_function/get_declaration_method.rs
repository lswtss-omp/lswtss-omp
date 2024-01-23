use std::ffi::c_char;
use std::ffi::c_void;

/// Hints:
/// - vtable_analysis.txt
const RAW_AS_I_SCRIPT_FUNCTION_GET_DECLARATION_METHOD_VTABLE_INDEX: isize = 15;

pub type RawAsIScriptFunctionGetDeclarationMethod = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    include_object_name: c_char,
    include_namespace: c_char,
    include_param_names: c_char,
) -> *mut c_char;

pub unsafe fn raw_as_i_script_function_get_declaration_method(
    handle: *mut c_void,
    include_object_name: c_char,
    include_namespace: c_char,
    include_param_names: c_char,
) -> *mut c_char
{
    let vtable = *(handle as *mut *mut *mut c_void);

    let method: RawAsIScriptFunctionGetDeclarationMethod = std::mem::transmute(
        *(vtable.byte_offset((RAW_AS_I_SCRIPT_FUNCTION_GET_DECLARATION_METHOD_VTABLE_INDEX) * 8)),
    );

    return method(
        handle,
        include_object_name,
        include_namespace,
        include_param_names,
    );
}

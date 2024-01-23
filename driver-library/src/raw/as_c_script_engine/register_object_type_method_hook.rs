use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawAsCScriptEngineRegisterObjectTypeMethod,
    RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_PTR_OPTION,
    RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_HOOK_TRAMPOLINE,
    raw_as_c_script_engine_register_object_type_method_hook_detour,
    register_raw_as_c_script_engine_register_object_type_method_hook,
);

pub unsafe extern "stdcall" fn raw_as_c_script_engine_register_object_type_method_hook_detour(
    handle: *mut c_void,
    name: *mut c_char,
    byte_size: c_int,
    flags: c_int,
) -> c_uint
{
    if RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.is_none() {
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION = Some(handle);

        log::info!(
            "RAW_AS_C_SCRIPT_ENGINE_HANDLE: {:?}",
            RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        );

        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION =
            Some(raw_as_i_script_engine_create_context_method(handle));

        log::info!(
            "RAW_AS_I_SCRIPT_CONTEXT_HANDLE: {:?}",
            RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        );
    }

    return RAW_AS_C_SCRIPT_ENGINE_REGISTER_OBJECT_TYPE_METHOD_HOOK_TRAMPOLINE(
        handle, name, byte_size, flags,
    );
}

use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawGameFrameworkVtable8Method,
    RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_PTR_OPTION,
    RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_HOOK_TRAMPOLINE,
    raw_game_framework_vtable_8_method_hook_detour,
    register_raw_game_framework_vtable_8_method_hook,
);

pub unsafe extern "stdcall" fn raw_game_framework_vtable_8_method_hook_detour(
    handle: *mut c_void
) -> bool
{
    test_api::process_requests();

    return RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_HOOK_TRAMPOLINE(handle);
}

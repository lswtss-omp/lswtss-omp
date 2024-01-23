use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawApiComponentEnableMethod,
    RAW_API_COMPONENT_ENABLE_METHOD_PTR_OPTION,
    RAW_API_COMPONENT_ENABLE_METHOD_HOOK_TRAMPOLINE,
    raw_api_component_enable_method_hook_detour,
    register_raw_api_component_enable_method_hook,
);

pub unsafe extern "stdcall" fn raw_api_component_enable_method_hook_detour(handle: *mut c_void)
{
    let raw_api_entity_handle = RAW_API_COMPONENT_GET_PARENT_METHOD_PTR_OPTION.unwrap()(handle);

    if !raw_api_entity_handle.is_null() {
        let raw_ntt_universe_handle =
            RAW_API_ENTITY_GET_UNIVERSE_METHOD_PTR_OPTION.unwrap()(raw_api_entity_handle);

        if !raw_ntt_universe_handle.is_null() {
            if let Some(raw_ntt_current_universe_handle) = RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION {
                if raw_ntt_current_universe_handle != raw_ntt_universe_handle {
                    RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION = Some(raw_ntt_universe_handle);
                    log::info!(
                        "RAW_NTT_CURRENT_UNIVERSE_HANDLE: {:?}",
                        RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION.unwrap()
                    );
                }
            } else {
                RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION = Some(raw_ntt_universe_handle);
                log::info!(
                    "RAW_NTT_CURRENT_UNIVERSE_HANDLE: {:?}",
                    RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION.unwrap()
                );
            }
        }
    }

    return RAW_API_COMPONENT_ENABLE_METHOD_HOOK_TRAMPOLINE(handle);
}

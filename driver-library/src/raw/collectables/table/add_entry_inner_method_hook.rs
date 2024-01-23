use std::ffi::c_char;
use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawCollectablesTableAddEntryInnerMethod,
    RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_PTR_OPTION,
    RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_HOOK_TRAMPOLINE,
    raw_collectables_table_add_entry_inner_method_hook_detour,
    register_raw_collectables_table_add_entry_inner_method_hook,
);

pub unsafe extern "stdcall" fn raw_collectables_table_add_entry_inner_method_hook_detour(
    handle: *mut c_void,
    name_as_c_char_ptr_ptr: *mut *mut c_char,
    guid_ptr: *mut RawNuGuid,
    dlc_guid_ptr: *mut RawNuGuid,
) -> *mut c_void
{
    let name_as_str = std::ffi::CStr::from_ptr(*name_as_c_char_ptr_ptr)
        .to_str()
        .unwrap();

    for custom_character_info in CUSTOM_CHARACTERS_INFO.iter() {
        if custom_character_info.id == name_as_str {
            let mut guid = get_raw_nu_guid_from_custom_character_id(&custom_character_info.id);

            return RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_HOOK_TRAMPOLINE(
                handle,
                name_as_c_char_ptr_ptr,
                &mut guid,
                dlc_guid_ptr,
            );
        }
    }

    return RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_HOOK_TRAMPOLINE(
        handle,
        name_as_c_char_ptr_ptr,
        guid_ptr,
        dlc_guid_ptr,
    );
}

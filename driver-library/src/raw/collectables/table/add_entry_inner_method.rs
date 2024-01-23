use std::ffi::c_char;
use std::ffi::c_void;

use crate::*;

/// Hints:
/// - called at the end of collectables::Table::AddEntry
const RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_OFFSET: isize = 0x157c080;

pub type RawCollectablesTableAddEntryInnerMethod = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    name_as_c_char_ptr_ptr: *mut *mut c_char,
    guid_ptr: *mut RawNuGuid,
    dlc_guid_ptr: *mut RawNuGuid,
) -> *mut c_void;

pub static mut RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_PTR_OPTION: Option<
    RawCollectablesTableAddEntryInnerMethod,
> = None;

pub fn register_raw_collectables_table_add_entry_inner_method_ptr(
    process_exe_module_handle: *mut c_void
)
{
    unsafe {
        RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle
                    .byte_offset(RAW_COLLECTABLES_TABLE_ADD_ENTRY_INNER_METHOD_OFFSET),
            ),
        );
    }
}

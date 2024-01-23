use std::ffi::c_void;

/// Hints:
/// - references `collectables::Table::vftable`
const RAW_COLLECTABLES_TABLE_CONSTRUCTOR_OFFSET: isize = 0x1578c30;

pub type RawCollectablesTableConstructor = unsafe extern "stdcall" fn(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
    unknown_param_4: *mut c_void,
) -> *mut c_void;

pub static mut RAW_COLLECTABLES_TABLE_CONSTRUCTOR_PTR_OPTION: Option<
    RawCollectablesTableConstructor,
> = None;

pub fn register_raw_collectables_table_constructor_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_COLLECTABLES_TABLE_CONSTRUCTOR_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_COLLECTABLES_TABLE_CONSTRUCTOR_OFFSET),
            ),
        );
    }
}

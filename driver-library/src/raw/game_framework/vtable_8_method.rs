use std::ffi::c_void;

/// Hints:
/// - registered as `GameFramework::vftable[8]`
/// - gets called in a loop on main process thread
const RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_OFFSET: isize = 0x366aca0;

pub type RawGameFrameworkVtable8Method = unsafe extern "stdcall" fn(handle: *mut c_void) -> bool;

pub static mut RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_PTR_OPTION: Option<
    RawGameFrameworkVtable8Method,
> = None;

pub fn register_raw_game_framework_vtable_8_method_ptr(process_exe_module_handle: *mut c_void)
{
    unsafe {
        RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_PTR_OPTION = Some(
            std::mem::transmute(
                process_exe_module_handle.byte_offset(RAW_GAME_FRAMEWORK_VTABLE_8_METHOD_OFFSET),
            ),
        );
    }
}

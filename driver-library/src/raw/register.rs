use std::ffi::c_void;

use crate::*;

pub fn register_raw(process_exe_module_handle: *mut c_void)
{
    register_raw_api_component_enable_method_ptr(process_exe_module_handle);
    register_raw_api_component_enable_method_hook();
    register_raw_api_component_get_parent_method_ptr(process_exe_module_handle);

    register_raw_api_entity_get_universe_method_ptr(process_exe_module_handle);

    register_raw_as_c_script_engine_register_object_type_method_ptr(process_exe_module_handle);
    register_raw_as_c_script_engine_register_object_type_method_hook();

    register_raw_collectables_table_add_entry_inner_method_ptr(process_exe_module_handle);
    register_raw_collectables_table_add_entry_inner_method_hook();
    register_raw_collectables_table_constructor_ptr(process_exe_module_handle);
    register_raw_collectables_table_constructor_hook();

    register_raw_game_framework_vtable_8_method_ptr(process_exe_module_handle);
    register_raw_game_framework_vtable_8_method_hook();

    register_raw_ntt_file_file_device_pc_constructor_ptr(process_exe_module_handle);
    register_raw_ntt_file_file_device_pc_constructor_hook();
    register_raw_ntt_file_file_device_pc_vtable_24_method_ptr(process_exe_module_handle);

    register_raw_nu_file_device_dat_constructor_ptr(process_exe_module_handle);
    register_raw_nu_file_device_dat_constructor_hook();
    register_raw_nu_file_device_dat_vtable_24_method_ptr(process_exe_module_handle);
    register_raw_nu_file_device_dat_vtable_24_method_hook();
    register_raw_nu_file_device_dat_vtable_31_method_ptr(process_exe_module_handle);
    register_raw_nu_file_device_dat_vtable_31_method_hook();
}

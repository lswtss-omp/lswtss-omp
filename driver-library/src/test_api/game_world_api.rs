use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::null_mut;

use crate::*;

pub unsafe fn find_game_world_entity(
    game_world_handle: *mut c_void,
    probably_game_world_entity_name: &str,
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        34,
    );

    let as_i_script_function_index = 2;

    let as_i_script_function_handle = raw_as_i_type_info_get_method_by_index_method(
        as_i_type_info_handle,
        as_i_script_function_index,
        1,
    );

    raw_as_i_script_context_prepare_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        as_i_script_function_handle,
    );

    raw_as_i_script_context_set_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        game_world_handle,
    );

    let game_component_type_name_as_cstring =
        CString::new(probably_game_world_entity_name).unwrap();
    let game_component_type_name_as_char_ptr = game_component_type_name_as_cstring.as_ptr();
    let game_component_type_name_as_char_ptr_ptr =
        &game_component_type_name_as_char_ptr as *const *const i8 as *mut c_void;

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_component_type_name_as_char_ptr_ptr,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(find_game_world_entity),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

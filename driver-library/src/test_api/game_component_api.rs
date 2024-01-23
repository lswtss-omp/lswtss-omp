use std::ffi::c_char;
use std::ffi::c_void;
use std::ptr::null_mut;

use crate::*;

pub unsafe fn get_game_component_parent_handle(game_component_handle: *mut c_void) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        32,
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
        game_component_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_component_parent_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_component_label(game_component_handle: *mut c_void) -> String
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        32,
    );

    let as_i_script_function_index = 9;

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
        game_component_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_component_label),
            as_i_script_context_execution_result
        );
        return "SCRIPT_ERROR".to_string();
    }

    let result_as_char_ptr_ptr = raw_as_i_script_context_get_return_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    ) as *mut *mut c_char;

    let result_as_char_ptr = *result_as_char_ptr_ptr;

    let result = std::ffi::CStr::from_ptr(result_as_char_ptr)
        .to_str()
        .unwrap()
        .to_owned();

    return result;
}

pub unsafe fn get_game_component_previous_sibling_handle(
    game_component_handle: *mut c_void
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        32,
    );

    let as_i_script_function_index = 10;

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
        game_component_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_component_previous_sibling_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_component_next_sibling_handle(
    game_component_handle: *mut c_void
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        32,
    );

    let as_i_script_function_index = 11;

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
        game_component_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_component_next_sibling_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

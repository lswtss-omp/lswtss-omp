use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::null_mut;

use crate::*;

pub unsafe fn get_game_entity_name(game_entity_handle: *mut c_void) -> String
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
    );

    let as_i_script_function_index = 0;

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
        game_entity_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_entity_name),
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

pub unsafe fn find_game_entity_child_entity(
    game_entity_handle: *mut c_void,
    probably_game_entity_child_entity_name: &str,
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
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
        game_entity_handle,
    );

    let game_component_type_name_as_cstring =
        CString::new(probably_game_entity_child_entity_name).unwrap();
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
            stringify!(find_game_entity_child_entity),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn find_game_entity_component(
    game_entity_handle: *mut c_void,
    probably_game_entity_component_name: &str,
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
    );

    let as_i_script_function_index = 3;

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
        game_entity_handle,
    );

    let game_component_type_name_as_cstring =
        CString::new(probably_game_entity_component_name).unwrap();
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
            stringify!(find_game_entity_component),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn find_game_entity_component_by_type_name(
    game_entity_handle: *mut c_void,
    game_component_type_name: &str,
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
    );

    let as_i_script_function_index = 4;

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
        game_entity_handle,
    );

    let game_component_type_name_as_cstring = CString::new(game_component_type_name).unwrap();
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
            stringify!(find_game_entity_component_by_type_name),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn find_game_entity_component_by_type_name_recursive(
    game_entity_handle: *mut c_void,
    game_component_type_name: &str,
    unknown_bool: bool,
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
    );

    let as_i_script_function_index = 5;

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
        game_entity_handle,
    );

    let game_component_type_name_as_cstring = CString::new(game_component_type_name).unwrap();
    let game_component_type_name_as_char_ptr = game_component_type_name_as_cstring.as_ptr();
    let game_component_type_name_as_char_ptr_ptr =
        &game_component_type_name_as_char_ptr as *const *const i8 as *mut c_void;

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_component_type_name_as_char_ptr_ptr,
    );

    raw_as_i_script_context_set_arg_byte_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        std::mem::transmute(unknown_bool),
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(find_game_entity_component_by_type_name_recursive),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_entity_world_handle(game_entity_handle: *mut c_void) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        33,
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
        game_entity_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_entity_world_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

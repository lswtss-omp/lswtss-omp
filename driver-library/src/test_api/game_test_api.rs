use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::null_mut;

use crate::*;

pub unsafe fn get_game_collectables_table_handle(
    game_collectables_table_name: String
) -> *mut c_void
{
    let as_i_script_function_handle = raw_as_i_script_engine_get_global_function_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        666,
    );

    raw_as_i_script_context_prepare_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        as_i_script_function_handle,
    );

    let game_collectables_table_name_as_cstring =
        CString::new(game_collectables_table_name).unwrap();
    let game_collectables_table_name_as_char_ptr = game_collectables_table_name_as_cstring.as_ptr();
    let game_collectables_table_name_as_char_ptr_ptr =
        &game_collectables_table_name_as_char_ptr as *const *const i8 as *mut c_void;

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_collectables_table_name_as_char_ptr_ptr,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_collectables_table_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_collectables_table_columns_count(
    game_collectable_table_handle: *mut c_void
) -> c_int
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
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
        game_collectable_table_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_collectables_table_columns_count),
            as_i_script_context_execution_result
        );
        return -1;
    }

    let result = raw_as_i_script_context_get_return_d_word_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return std::mem::transmute(result);
}

pub unsafe fn get_game_collectables_table_column_name(
    game_collectable_table_handle: *mut c_void,
    game_collectable_table_column_index: i32,
) -> String
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
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
        game_collectable_table_handle,
    );

    raw_as_i_script_context_set_arg_d_word_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        std::mem::transmute(game_collectable_table_column_index),
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_collectables_table_column_name),
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

pub unsafe fn get_game_collectables_table_name(game_collectable_table_handle: *mut c_void)
    -> String
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
    );

    let as_i_script_function_index = 15;

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
        game_collectable_table_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_collectables_table_name),
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

pub unsafe fn add_game_collectables_table_entry(
    game_collectables_table_handle: *mut c_void,
    game_collectables_table_entry_name: &str,
    game_collectables_table_entry_dlc_guid: &[u8; 17],
) -> *mut c_void
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
    );

    let as_i_script_function_index = 0;

    let as_i_script_function_handle = raw_as_i_type_info_get_method_by_index_method(
        as_i_type_info_handle,
        as_i_script_function_index,
        0,
    );

    raw_as_i_script_context_prepare_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        as_i_script_function_handle,
    );

    raw_as_i_script_context_set_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        game_collectables_table_handle,
    );

    let game_collectables_table_entry_name_as_cstring =
        CString::new(game_collectables_table_entry_name).unwrap();
    let game_collectables_table_entry_name_as_char_ptr =
        game_collectables_table_entry_name_as_cstring.as_ptr();
    let game_collectables_table_entry_name_as_char_ptr_ptr =
        &game_collectables_table_entry_name_as_char_ptr as *const *const i8 as *mut c_void;

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_collectables_table_entry_name_as_char_ptr_ptr,
    );

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        game_collectables_table_entry_dlc_guid.as_ptr() as *mut c_void,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(add_game_collectables_table_entry),
            as_i_script_context_execution_result
        );

        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn remove_game_collectables_table_entry(
    game_collectables_table_handle: *mut c_void,
    game_collectables_table_entry_name: &str,
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
    );

    let as_i_script_function_index = 1;

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
        game_collectables_table_handle,
    );

    let game_collectables_table_entry_name_as_cstring =
        CString::new(game_collectables_table_entry_name).unwrap();
    let game_collectables_table_entry_name_as_char_ptr =
        game_collectables_table_entry_name_as_cstring.as_ptr();
    let game_collectables_table_entry_name_as_char_ptr_ptr =
        &game_collectables_table_entry_name_as_char_ptr as *const *const i8 as *mut c_void;

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_collectables_table_entry_name_as_char_ptr_ptr,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(remove_game_collectables_table_entry),
            as_i_script_context_execution_result
        );
    }
}

pub unsafe fn get_game_collectables_table_entries_count(
    game_collectable_table_handle: *mut c_void
) -> c_int
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
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
        game_collectable_table_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_collectables_table_columns_count),
            as_i_script_context_execution_result
        );
        return -1;
    }

    let result = raw_as_i_script_context_get_return_d_word_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return std::mem::transmute(result);
}

pub unsafe fn on_game_collectables_table_structure_changed(
    game_collectables_table_handle: *mut c_void
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1202,
    );

    let as_i_script_function_index = 13;

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
        game_collectables_table_handle,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(on_game_collectables_table_structure_changed),
            as_i_script_context_execution_result
        );
    }
}

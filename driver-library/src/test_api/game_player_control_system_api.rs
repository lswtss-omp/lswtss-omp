use std::ffi::c_void;
use std::ptr::null_mut;

use crate::*;

pub unsafe fn get_game_player_control_system_handle() -> *mut c_void
{
    let as_i_script_function_handle = raw_as_i_script_engine_get_global_function_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        1342,
    );

    raw_as_i_script_context_prepare_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        as_i_script_function_handle,
    );

    raw_as_i_script_context_set_arg_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION.unwrap(),
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_player_control_system_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_player_controlled_entity_handle(game_player_id: i32) -> *mut c_void
{
    let game_player_system_handle = get_game_player_control_system_handle();

    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        4548,
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
        game_player_system_handle,
    );

    raw_as_i_script_context_set_arg_d_word_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        std::mem::transmute(game_player_id),
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_player_controlled_entity_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

pub unsafe fn get_game_player_entity_handle(game_player_id: i32) -> *mut c_void
{
    let game_player_system_handle = get_game_player_control_system_handle();

    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        4548,
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
        game_player_system_handle,
    );

    raw_as_i_script_context_set_arg_d_word_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        std::mem::transmute(game_player_id),
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_player_entity_handle),
            as_i_script_context_execution_result
        );
        return null_mut();
    }

    let result = raw_as_i_script_context_get_return_object_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
    );

    return result;
}

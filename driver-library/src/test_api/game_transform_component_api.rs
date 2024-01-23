use std::ffi::c_void;

use crate::*;

pub unsafe fn set_game_transform_component_position(
    game_transform_component_handle: *mut c_void,
    game_transform_component_position: (
        f32,
        f32,
        f32,
    ),
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        103,
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
        game_transform_component_handle,
    );

    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_transform_component_position.0,
    );
    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        game_transform_component_position.1,
    );
    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        2,
        game_transform_component_position.2,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(set_game_transform_component_position),
            as_i_script_context_execution_result
        );
    }
}

pub unsafe fn set_game_transform_component_scale(
    game_transform_component_handle: *mut c_void,
    game_transform_component_scale: (
        f32,
        f32,
        f32,
    ),
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        103,
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
        game_transform_component_handle,
    );

    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        game_transform_component_scale.0,
    );
    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        game_transform_component_scale.0,
    );
    raw_as_i_script_context_set_arg_float_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        2,
        game_transform_component_scale.0,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(set_game_transform_component_scale),
            as_i_script_context_execution_result
        );
    }
}

pub unsafe fn get_game_transform_component_position(
    game_transform_component_handle: *mut c_void
) -> (
    f32,
    f32,
    f32,
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        103,
    );

    let as_i_script_function_index = 7;

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
        game_transform_component_handle,
    );

    let mut result_x: f32 = 0.0;
    let mut result_y: f32 = 0.0;
    let mut result_z: f32 = 0.0;

    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        &mut result_x as *mut f32 as *mut c_void,
    );
    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        &mut result_y as *mut f32 as *mut c_void,
    );
    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        2,
        &mut result_z as *mut f32 as *mut c_void,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_transform_component_position),
            as_i_script_context_execution_result
        );
        return (
            0.0, 0.0, 0.0,
        );
    }

    return (
        result_x, result_y, result_z,
    );
}

pub unsafe fn get_game_transform_component_scale(
    game_transform_component_handle: *mut c_void
) -> (
    f32,
    f32,
    f32,
)
{
    let as_i_type_info_handle = raw_as_i_script_engine_get_object_type_by_index_method(
        RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.unwrap(),
        103,
    );

    let as_i_script_function_index = 8;

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
        game_transform_component_handle,
    );

    let mut result_x: f32 = 0.0;
    let mut result_y: f32 = 0.0;
    let mut result_z: f32 = 0.0;

    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        0,
        &mut result_x as *mut f32 as *mut c_void,
    );
    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        1,
        &mut result_y as *mut f32 as *mut c_void,
    );
    raw_as_i_script_context_set_arg_address_method(
        RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap(),
        2,
        &mut result_z as *mut f32 as *mut c_void,
    );

    let as_i_script_context_execution_result =
        raw_as_i_script_context_execute_method(RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.unwrap());

    if as_i_script_context_execution_result != 0 {
        log::info!(
            "{} failed with: {}",
            stringify!(get_game_transform_component_scale),
            as_i_script_context_execution_result
        );
        return (
            0.0, 0.0, 0.0,
        );
    }

    return (
        result_x, result_y, result_z,
    );
}

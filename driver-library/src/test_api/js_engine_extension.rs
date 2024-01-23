use std::ffi::c_void;
use std::sync::Mutex;

use deno_runtime::deno_core;

use super::*;

lazy_static::lazy_static! {
  pub static ref TEST_MUTEX: Mutex<u32> = Mutex::new(0);
}

#[deno_core::op2(async)]
pub async fn op_find_game_world_entity(
    game_world_handle: *mut c_void,
    #[string] probably_game_world_entity_name: String,
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::FindGameWorldEntity(
        FindGameWorldEntityRequest {
            game_world_handle,
            probably_game_world_entity_name,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_game_player_controlled_entity_handle(game_player_id: i32) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGamePlayerControlledEntityHandle(
        GetGamePlayerControlledEntityHandleRequest {
            game_player_id,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_game_player_entity_handle(game_player_id: i32) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGamePlayerEntityHandle(
        GetGamePlayerEntityHandleRequest {
            game_player_id,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
#[string]
pub async fn op_get_game_entity_name(game_entity_handle: *mut c_void) -> String
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<String>();

    let request = RequestInfo::GetGameEntityName(
        GetGameEntityNameRequest {
            game_entity_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_find_game_entity_child_entity(
    game_entity_handle: *mut c_void,
    #[string] probably_game_entity_child_entity_name: String,
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::FindGameEntityChildEntity(
        FindGameEntityChildEntityRequest {
            game_entity_handle,
            probably_game_entity_child_entity_name,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_find_game_entity_component(
    game_entity_handle: *mut c_void,
    #[string] probably_game_entity_component_name: String,
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::FindGameEntityComponent(
        FindGameEntityComponentRequest {
            game_entity_handle,
            probably_game_entity_component_name,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_find_game_entity_component_by_type_name(
    game_entity_handle: *mut c_void,
    #[string] game_entity_component_type_name: String,
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::FindGameEntityComponentByTypeName(
        FindGameEntityComponentByTypeNameRequest {
            game_entity_handle,
            game_entity_component_type_name,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_find_game_entity_component_by_type_name_recursive(
    game_entity_handle: *mut c_void,
    #[string] game_entity_component_type_name: String,
    unknown_bool: bool,
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::FindGameEntityComponentByTypeNameRecursive(
        FindGameEntityComponentByTypeNameRecursiveRequest {
            game_entity_handle,
            game_entity_component_type_name,
            unknown_bool,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_game_entity_world_handle(game_entity_handle: *mut c_void) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGameEntityWorldHandle(
        GetGameEntityWorldHandleRequest {
            game_entity_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_component_parent_handle(game_component_handle: *mut c_void) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGameComponentParentHandle(
        GetGameComponentParentHandleRequest {
            game_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
#[string]
pub async fn op_get_game_component_label(game_component_handle: *mut c_void) -> String
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<String>();

    let request = RequestInfo::GetGameComponentLabel(
        GetGameComponentLabelRequest {
            game_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_game_component_next_sibling_handle(
    game_component_handle: *mut c_void
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGameComponentNextSiblingHandle(
        GetGameComponentNextSiblingHandleRequest {
            game_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_get_game_component_previous_sibling_handle(
    game_component_handle: *mut c_void
) -> *mut c_void
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<*mut c_void>();

    let request = RequestInfo::GetGameComponentPreviousSiblingHandle(
        GetGameComponentPreviousSiblingHandleRequest {
            game_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
#[serde]
pub async fn op_get_game_transform_component_position(
    game_transform_component_handle: *mut c_void
) -> (
    f32,
    f32,
    f32,
)
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<(
            f32,
            f32,
            f32,
        )>();

    let request = RequestInfo::GetGameTransformComponentPosition(
        GetGameTransformComponentPositionRequest {
            game_transform_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );

    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
#[serde]
pub async fn op_get_game_transform_component_scale(
    game_transform_component_handle: *mut c_void
) -> (
    f32,
    f32,
    f32,
)
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<(
            f32,
            f32,
            f32,
        )>();

    let request = RequestInfo::GetGameTransformComponentScale(
        GetGameTransformComponentScaleRequest {
            game_transform_component_handle,
            response_channel_sender: request_response_channel_sender,
        },
    );
    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    let response = request_response_channel_receiver
        .await
        .unwrap();

    return response;
}

#[deno_core::op2(async)]
pub async fn op_set_game_transform_component_position(
    game_transform_component_handle: *mut c_void,
    #[serde] game_transform_component_position: (
        f32,
        f32,
        f32,
    ),
)
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<()>();

    let request = RequestInfo::SetGameTransformComponentPosition(
        SetGameTransformComponentPositionRequest {
            game_transform_component_handle,
            game_transform_component_position,
            response_channel_sender: request_response_channel_sender,
        },
    );
    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };

    request_response_channel_receiver
        .await
        .unwrap();
}

#[deno_core::op2(async)]
pub async fn op_set_game_transform_component_scale(
    game_transform_component_handle: *mut c_void,
    #[serde] game_transform_component_scale: (
        f32,
        f32,
        f32,
    ),
)
{
    let (request_response_channel_sender, request_response_channel_receiver) =
        tokio::sync::oneshot::channel::<()>();

    let request = RequestInfo::SetGameTransformComponentScale(
        SetGameTransformComponentScaleRequest {
            game_transform_component_handle,
            game_transform_component_scale,
            response_channel_sender: request_response_channel_sender,
        },
    );
    let _ = unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION
            .as_ref()
            .unwrap()
            .send(request)
    };
    request_response_channel_receiver
        .await
        .unwrap();
}

deno_core::extension!(
    test_extension,
    ops = [
        op_find_game_world_entity,
        op_get_game_player_controlled_entity_handle,
        op_get_game_player_entity_handle,
        op_get_game_entity_name,
        op_find_game_entity_child_entity,
        op_find_game_entity_component,
        op_find_game_entity_component_by_type_name,
        op_find_game_entity_component_by_type_name_recursive,
        op_get_game_entity_world_handle,
        op_get_component_parent_handle,
        op_get_game_component_label,
        op_get_game_component_previous_sibling_handle,
        op_get_game_component_next_sibling_handle,
        op_get_game_transform_component_position,
        op_get_game_transform_component_scale,
        op_set_game_transform_component_position,
        op_set_game_transform_component_scale,
    ],
    esm_entry_point = "ext:test_extension/src/test_api/js_engine_extension.js",
    esm = ["src/test_api/js_engine_extension.js"],
);

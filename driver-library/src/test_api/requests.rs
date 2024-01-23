use std::ffi::c_void;

pub struct FindGameWorldEntityRequest
{
    pub game_world_handle: *mut c_void,
    pub probably_game_world_entity_name: String,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGamePlayerControlledEntityHandleRequest
{
    pub game_player_id: i32,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGamePlayerEntityHandleRequest
{
    pub game_player_id: i32,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGameEntityNameRequest
{
    pub game_entity_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<String>,
}

pub struct FindGameEntityChildEntityRequest
{
    pub game_entity_handle: *mut c_void,
    pub probably_game_entity_child_entity_name: String,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct FindGameEntityComponentRequest
{
    pub game_entity_handle: *mut c_void,
    pub probably_game_entity_component_name: String,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct FindGameEntityComponentByTypeNameRequest
{
    pub game_entity_handle: *mut c_void,
    pub game_entity_component_type_name: String,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct FindGameEntityComponentByTypeNameRecursiveRequest
{
    pub game_entity_handle: *mut c_void,
    pub game_entity_component_type_name: String,
    pub unknown_bool: bool,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGameEntityWorldHandleRequest
{
    pub game_entity_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGameComponentParentHandleRequest
{
    pub game_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGameComponentLabelRequest
{
    pub game_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<String>,
}

pub struct GetGameComponentPreviousSiblingHandleRequest
{
    pub game_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct GetGameComponentNextSiblingHandleRequest
{
    pub game_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<*mut c_void>,
}

pub struct SetGameTransformComponentPositionRequest
{
    pub game_transform_component_handle: *mut c_void,
    pub game_transform_component_position: (
        f32,
        f32,
        f32,
    ),
    pub response_channel_sender: tokio::sync::oneshot::Sender<()>,
}

pub struct SetGameTransformComponentScaleRequest
{
    pub game_transform_component_handle: *mut c_void,
    pub game_transform_component_scale: (
        f32,
        f32,
        f32,
    ),
    pub response_channel_sender: tokio::sync::oneshot::Sender<()>,
}

pub struct GetGameTransformComponentPositionRequest
{
    pub game_transform_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<(
        f32,
        f32,
        f32,
    )>,
}

pub struct GetGameTransformComponentScaleRequest
{
    pub game_transform_component_handle: *mut c_void,
    pub response_channel_sender: tokio::sync::oneshot::Sender<(
        f32,
        f32,
        f32,
    )>,
}

pub enum RequestInfo
{
    FindGameWorldEntity(FindGameWorldEntityRequest),
    GetGamePlayerControlledEntityHandle(GetGamePlayerControlledEntityHandleRequest),
    GetGamePlayerEntityHandle(GetGamePlayerEntityHandleRequest),
    GetGameEntityName(GetGameEntityNameRequest),
    FindGameEntityChildEntity(FindGameEntityChildEntityRequest),
    FindGameEntityComponent(FindGameEntityComponentRequest),
    FindGameEntityComponentByTypeName(FindGameEntityComponentByTypeNameRequest),
    FindGameEntityComponentByTypeNameRecursive(FindGameEntityComponentByTypeNameRecursiveRequest),
    GetGameEntityWorldHandle(GetGameEntityWorldHandleRequest),
    GetGameComponentParentHandle(GetGameComponentParentHandleRequest),
    GetGameComponentLabel(GetGameComponentLabelRequest),
    GetGameComponentPreviousSiblingHandle(GetGameComponentPreviousSiblingHandleRequest),
    GetGameComponentNextSiblingHandle(GetGameComponentNextSiblingHandleRequest),
    SetGameTransformComponentPosition(SetGameTransformComponentPositionRequest),
    SetGameTransformComponentScale(SetGameTransformComponentScaleRequest),
    GetGameTransformComponentPosition(GetGameTransformComponentPositionRequest),
    GetGameTransformComponentScale(GetGameTransformComponentScaleRequest),
}

pub static mut REQUESTS_CHANNEL_SENDER_OPTION: Option<
    tokio::sync::mpsc::UnboundedSender<RequestInfo>,
> = None;

pub static mut REQUESTS_CHANNEL_RECEIVER_OPTION: Option<
    tokio::sync::mpsc::UnboundedReceiver<RequestInfo>,
> = None;

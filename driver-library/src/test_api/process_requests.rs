use super::*;

pub unsafe fn process_requests()
{
    if !is_game_api_ready() {
        return;
    }

    while let Ok(request) = REQUESTS_CHANNEL_RECEIVER_OPTION
        .as_mut()
        .unwrap()
        .try_recv()
    {
        match request {
            RequestInfo::FindGameWorldEntity(find_game_world_entity_request) => {
                let response = find_game_world_entity(
                    find_game_world_entity_request.game_world_handle,
                    &find_game_world_entity_request.probably_game_world_entity_name,
                );

                let _ = find_game_world_entity_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGamePlayerControlledEntityHandle(
                get_game_player_controlled_entity_handle_request,
            ) => {
                let response = get_game_player_controlled_entity_handle(
                    get_game_player_controlled_entity_handle_request.game_player_id,
                );

                let _ = get_game_player_controlled_entity_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGamePlayerEntityHandle(get_game_player_entity_handle_request) => {
                let response = get_game_player_entity_handle(
                    get_game_player_entity_handle_request.game_player_id,
                );

                let _ = get_game_player_entity_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameEntityName(get_game_entity_name_request) => {
                let response =
                    get_game_entity_name(get_game_entity_name_request.game_entity_handle);

                let _ = get_game_entity_name_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::FindGameEntityChildEntity(
                find_game_entity_child_entity_handle_request,
            ) => {
                let response = find_game_entity_child_entity(
                    find_game_entity_child_entity_handle_request.game_entity_handle,
                    &find_game_entity_child_entity_handle_request
                        .probably_game_entity_child_entity_name,
                );

                let _ = find_game_entity_child_entity_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::FindGameEntityComponent(find_game_entity_component_request) => {
                let response = find_game_entity_component(
                    find_game_entity_component_request.game_entity_handle,
                    &find_game_entity_component_request.probably_game_entity_component_name,
                );

                let _ = find_game_entity_component_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::FindGameEntityComponentByTypeName(
                find_game_entity_component_by_type_name_request,
            ) => {
                let response = find_game_entity_component_by_type_name(
                    find_game_entity_component_by_type_name_request.game_entity_handle,
                    &find_game_entity_component_by_type_name_request
                        .game_entity_component_type_name,
                );

                let _ = find_game_entity_component_by_type_name_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::FindGameEntityComponentByTypeNameRecursive(
                find_game_entity_component_by_type_name_recursive_request,
            ) => {
                let response = find_game_entity_component_by_type_name_recursive(
                    find_game_entity_component_by_type_name_recursive_request.game_entity_handle,
                    &find_game_entity_component_by_type_name_recursive_request
                        .game_entity_component_type_name,
                    find_game_entity_component_by_type_name_recursive_request.unknown_bool,
                );

                let _ = find_game_entity_component_by_type_name_recursive_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameEntityWorldHandle(get_game_entity_world_handle_request) => {
                let response = get_game_entity_world_handle(
                    get_game_entity_world_handle_request.game_entity_handle,
                );

                let _ = get_game_entity_world_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameComponentParentHandle(get_game_component_parent_handle_request) => {
                let response = get_game_component_parent_handle(
                    get_game_component_parent_handle_request.game_component_handle,
                );

                let _ = get_game_component_parent_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameComponentLabel(get_game_component_label_request) => {
                let response = get_game_component_label(
                    get_game_component_label_request.game_component_handle,
                );

                let _ = get_game_component_label_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameComponentPreviousSiblingHandle(
                get_game_component_previous_sibling_handle_request,
            ) => {
                let response = get_game_component_previous_sibling_handle(
                    get_game_component_previous_sibling_handle_request.game_component_handle,
                );

                let _ = get_game_component_previous_sibling_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameComponentNextSiblingHandle(
                get_game_component_next_sibling_handle_request,
            ) => {
                let response = get_game_component_next_sibling_handle(
                    get_game_component_next_sibling_handle_request.game_component_handle,
                );

                let _ = get_game_component_next_sibling_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::SetGameTransformComponentPosition(
                set_game_transform_component_position_handle_request,
            ) => {
                set_game_transform_component_position(
                    set_game_transform_component_position_handle_request
                        .game_transform_component_handle,
                    set_game_transform_component_position_handle_request
                        .game_transform_component_position,
                );

                let _ = set_game_transform_component_position_handle_request
                    .response_channel_sender
                    .send(());
            },
            RequestInfo::SetGameTransformComponentScale(
                set_game_transform_component_scale_handle_request,
            ) => {
                set_game_transform_component_scale(
                    set_game_transform_component_scale_handle_request
                        .game_transform_component_handle,
                    set_game_transform_component_scale_handle_request
                        .game_transform_component_scale,
                );

                let _ = set_game_transform_component_scale_handle_request
                    .response_channel_sender
                    .send(());
            },
            RequestInfo::GetGameTransformComponentPosition(
                get_game_transform_component_position_handle_request,
            ) => {
                let response = get_game_transform_component_position(
                    get_game_transform_component_position_handle_request
                        .game_transform_component_handle,
                );

                let _ = get_game_transform_component_position_handle_request
                    .response_channel_sender
                    .send(response);
            },
            RequestInfo::GetGameTransformComponentScale(
                get_game_transform_component_scale_handle_request,
            ) => {
                let response = get_game_transform_component_scale(
                    get_game_transform_component_scale_handle_request
                        .game_transform_component_handle,
                );

                let _ = get_game_transform_component_scale_handle_request
                    .response_channel_sender
                    .send(response);
            },
        }
    }
}

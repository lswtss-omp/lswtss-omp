import { core } from "ext:core/mod.js";

const {
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
} = core.ensureFastOps();

const TestAPI = {
  findWorldEntity: (worldHandle, probablyWorldEntityName) => {
    return op_find_game_world_entity(worldHandle, probablyWorldEntityName);
  },
  getPlayerControlledEntityHandle: (playerId) => {
    return op_get_game_player_controlled_entity_handle(playerId);
  },
  getPlayerEntityHandle: (playerId) => {
    return op_get_game_player_entity_handle(playerId);
  },
  getEntityName: (entityHandle) => {
    return op_get_game_entity_name(entityHandle);
  },
  findEntityChildEntity: (entityHandle, probablyChildEntityName) => {
    return op_find_game_entity_child_entity(
      entityHandle,
      probablyChildEntityName,
    );
  },
  findEntityComponent: (entityHandle, probablyComponentName) => {
    return op_find_game_entity_component(
      entityHandle,
      probablyComponentName,
    );
  },
  findEntityComponentByTypeName: (entityHandle, entityComponentTypeName) => {
    return op_find_game_entity_component_by_type_name(
      entityHandle,
      entityComponentTypeName,
    );
  },
  findEntityComponentByTypeNameRecursive: (
    entityHandle,
    entityComponentTypeName,
    unknownBool,
  ) => {
    return op_find_game_entity_component_by_type_name_recursive(
      entityHandle,
      entityComponentTypeName,
      unknownBool,
    );
  },
  getEntityWorldHandle: (entityHandle) => {
    return op_get_game_entity_world_handle(entityHandle);
  },
  getComponentParentHandle: (componentHandle) => {
    return op_get_component_parent_handle(componentHandle);
  },
  getComponentLabel: (componentHandle) => {
    return op_get_game_component_label(componentHandle);
  },
  getComponentPreviousSiblingHandle: (componentHandle) => {
    return op_get_game_component_previous_sibling_handle(componentHandle);
  },
  getComponentNextSiblingHandle: (componentHandle) => {
    return op_get_game_component_next_sibling_handle(componentHandle);
  },
  getTransformComponentPosition: (transformComponentHandle) => {
    return op_get_game_transform_component_position(transformComponentHandle);
  },
  getTransformComponentScale: (transformComponentHandle) => {
    return op_get_game_transform_component_scale(transformComponentHandle);
  },
  setTransformComponentPosition: (
    transformComponentHandle,
    transformComponentPosition,
  ) => {
    return op_set_game_transform_component_position(
      transformComponentHandle,
      transformComponentPosition,
    );
  },
  setTransformComponentScale: (
    transformComponentHandle,
    transformComponentScale,
  ) => {
    return op_set_game_transform_component_scale(
      transformComponentHandle,
      transformComponentScale,
    );
  },
};

globalThis.TestAPI = TestAPI;

export { TestAPI };

use std::ffi::c_void;

use crate::*;

declare_raw_fn_hook!(
    RawCollectablesTableConstructor,
    RAW_COLLECTABLES_TABLE_CONSTRUCTOR_PTR_OPTION,
    RAW_COLLECTABLES_TABLE_CONSTRUCTOR_HOOK_TRAMPOLINE,
    raw_collectables_table_constructor_hook_detour,
    register_raw_collectables_table_constructor_hook,
);

static mut WAS_RAW_COLLECTABLES_CHARACTERS_TABLE_PROCESSED: bool = false;

const RAW_COLLECTABLES_ENTRY_EMPTY_GUID: [u8; 17] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

pub unsafe extern "stdcall" fn raw_collectables_table_constructor_hook_detour(
    handle: *mut c_void,
    unknown_param_2: *mut c_void,
    unknown_param_3: *mut c_void,
    unknown_param_4: *mut c_void,
) -> *mut c_void
{
    for raw_collectable_table_handle_ref in &RAW_COLLECTABLE_TABLE_HANDLES {
        let raw_collectable_table_handle = *raw_collectable_table_handle_ref;

        let raw_collectables_table_name =
            test_api::get_game_collectables_table_name(raw_collectable_table_handle);

        if !WAS_RAW_COLLECTABLES_CHARACTERS_TABLE_PROCESSED
            && raw_collectables_table_name == "Characters"
        {
            for custom_character_info in &CUSTOM_CHARACTERS_INFO {
                let raw_collectables_table_new_entry_index =
                    test_api::get_game_collectables_table_entries_count(
                        raw_collectable_table_handle,
                    );

                test_api::add_game_collectables_table_entry(
                    raw_collectable_table_handle,
                    &custom_character_info.id,
                    &RAW_COLLECTABLES_ENTRY_EMPTY_GUID,
                );

                let raw_collectables_table_values_handle = (raw_collectable_table_handle
                    as *mut RawNuVector<RawCollectablesEntryValues>)
                    .byte_offset(0x140);

                let raw_collectables_table_new_entry_values_handle =
                    (*raw_collectables_table_values_handle)
                        .get_handle_at_index(raw_collectables_table_new_entry_index as usize);

                let raw_collectables_table_new_entry_collect_state_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(0))
                    .value as *mut RawCollectablesEnumVal;

                let raw_collectables_table_new_entry_graph_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(1))
                    .value as *mut RawCollectablesGraphVal;

                let raw_collectables_table_new_entry_character_class_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(2))
                    .value as *mut RawCollectablesEnumVal;

                let raw_collectables_table_new_entry_icon_file_name_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(3))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_display_string_id_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(4))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_description_string_id_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(5))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_is_new_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(6))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_base_character_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(7))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_currently_selected_outfit_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(8))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_price_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(9))
                    .value as *mut RawCollectablesIntVal;

                let raw_collectables_table_new_entry_character_base_name_id_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(10))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_is_playable_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(11))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_add_to_percentage_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(12))
                    .value as *mut RawCollectablesBoolVal;

                // let raw_collectables_table_new_entry_linked_characters_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(13))
                //     .value as *mut RawLinkedCharacterColumnData;

                // let raw_collectables_table_new_entry_collectable_info_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(14))
                //     .value as *mut RawCollectableInfoColumnData;

                let raw_collectables_table_new_entry_is_rideable_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(15))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_has_been_ridden_by_p1_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(16))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_has_been_ridden_by_p2_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(17))
                    .value as *mut RawCollectablesBoolVal;

                // let raw_collectables_table_new_entry_blip_data_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(18))
                //     .value as *mut RawIsNewColumnData;

                let raw_collectables_table_new_entry_cheat_code_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(19))
                    .value as *mut RawCollectablesStringVal;

                let raw_collectables_table_new_entry_chronology_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(20))
                    .value as *mut RawCollectablesIntVal;

                // let raw_collectables_table_new_entry_riding_info_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(21))
                //     .value as *mut RawCharacterSeatInfoColumnData;

                // let raw_collectables_table_new_entry_capsule_data_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(22))
                //     .value as *mut RawObjectExtentsColumn;

                let raw_collectables_table_new_entry_show_on_collect_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(23))
                    .value as *mut RawCollectablesBoolVal;

                // let raw_collectables_table_new_entry_cheat_codes_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(24))
                //     .value as *mut RawCheatCodeColumnData;

                let raw_collectables_table_new_entry_is_pop_character_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(25))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_is_quest_character_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(26))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_is_custom_or_costume_character_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(27))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_is_protocol_droid_segment_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(28))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_is_boss_character_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(29))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_cheap_graph_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(30))
                    .value as *mut RawCollectablesGraphVal;

                let raw_collectables_table_new_entry_immune_to_hazardous_gas_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(31))
                    .value as *mut RawCollectablesBoolVal;

                let raw_collectables_table_new_entry_show_on_party_bar_val_handle =
                    (*(*raw_collectables_table_new_entry_values_handle)
                        .values
                        .get_handle_at_index(32))
                    .value as *mut RawCollectablesBoolVal;

                // let raw_collectables_table_new_entry_associated_characters_val_handle =
                //     (*(*raw_collectables_table_new_entry_values_handle)
                //         .values
                //         .get_handle_at_index(33))
                //     .value as *mut RawAssociatedCharacterCustomData;

                (*raw_collectables_table_new_entry_collect_state_val_handle).value = 2;
                (*raw_collectables_table_new_entry_graph_val_handle)
                    .value
                    .set_resource_path_as_str(&custom_character_info.prefab_resource_path);
                (*raw_collectables_table_new_entry_character_class_val_handle).value =
                    match custom_character_info.class {
                        CustomCharacterClass::Jedi => 0,
                        CustomCharacterClass::Sith => 1,
                        CustomCharacterClass::RebelResistance => 2,
                        CustomCharacterClass::BountyHunter => 3,
                        CustomCharacterClass::AstromechDroid => 4,
                        CustomCharacterClass::ProtocolDroid => 5,
                        CustomCharacterClass::Scoundrel => 6,
                        CustomCharacterClass::GalacticEmpire => 7,
                        CustomCharacterClass::Scavenger => 8,
                        CustomCharacterClass::Civilian => 9,
                    };
                (*raw_collectables_table_new_entry_icon_file_name_val_handle)
                    .set_value_as_str("Silhouette_White.png");
                (*raw_collectables_table_new_entry_display_string_id_val_handle)
                    .set_value_as_str(&custom_character_info.name_string_id);
                (*raw_collectables_table_new_entry_description_string_id_val_handle)
                    .set_value_as_str(&custom_character_info.description_string_id);
                (*raw_collectables_table_new_entry_is_new_val_handle).value = 0;
                (*raw_collectables_table_new_entry_base_character_val_handle)
                    .set_value_as_str(&custom_character_info.id);
                (*raw_collectables_table_new_entry_currently_selected_outfit_val_handle).value = 1;
                (*raw_collectables_table_new_entry_price_val_handle).value = 0;
                (*raw_collectables_table_new_entry_character_base_name_id_val_handle)
                    .set_value_as_str(&custom_character_info.name_string_id);
                (*raw_collectables_table_new_entry_is_playable_val_handle).value = 1;
                (*raw_collectables_table_new_entry_add_to_percentage_val_handle).value = 0;
                (*raw_collectables_table_new_entry_is_rideable_val_handle).value = 0;
                (*raw_collectables_table_new_entry_has_been_ridden_by_p1_val_handle).value = 0;
                (*raw_collectables_table_new_entry_has_been_ridden_by_p2_val_handle).value = 0;
                (*raw_collectables_table_new_entry_cheat_code_val_handle).set_value_as_str("");
                (*raw_collectables_table_new_entry_chronology_val_handle).value = 0;
                (*raw_collectables_table_new_entry_show_on_collect_val_handle).value = 0;
                (*raw_collectables_table_new_entry_is_pop_character_val_handle).value = 0;
                (*raw_collectables_table_new_entry_is_quest_character_val_handle).value = 0;
                (*raw_collectables_table_new_entry_is_custom_or_costume_character_val_handle)
                    .value = 0;
                (*raw_collectables_table_new_entry_is_protocol_droid_segment_val_handle).value = 0;
                (*raw_collectables_table_new_entry_is_boss_character_val_handle).value = 0;
                (*raw_collectables_table_new_entry_cheap_graph_val_handle)
                    .value
                    .set_resource_path_as_str(&custom_character_info.preview_prefab_resource_path);
                (*raw_collectables_table_new_entry_immune_to_hazardous_gas_val_handle).value = 0;
                (*raw_collectables_table_new_entry_show_on_party_bar_val_handle).value = 0;
            }

            WAS_RAW_COLLECTABLES_CHARACTERS_TABLE_PROCESSED = true;
        }
    }

    RAW_COLLECTABLE_TABLE_HANDLES.push(handle);

    return RAW_COLLECTABLES_TABLE_CONSTRUCTOR_HOOK_TRAMPOLINE(
        handle,
        unknown_param_2,
        unknown_param_3,
        unknown_param_4,
    );
}

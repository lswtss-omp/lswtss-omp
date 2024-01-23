use crate::*;

pub fn execute_mod_action(
    mod_id: &str,
    mod_dir_path: &std::path::Path,
    mod_action_info: &ModActionInfo,
)
{
    match mod_action_info {
        ModActionInfo::ModV1RegisterCustomResourceActionInfo {
            payload: mod_v1_register_custom_resource_action_payload,
        } => unsafe {
            log::info!(
                "Executing v1/register-custom-resource-action for {}...",
                mod_id
            );
            CUSTOM_RESOURCES_INFO.push(
                CustomResourceInfo {
                    path: mod_v1_register_custom_resource_action_payload
                        .path
                        .clone(),
                    src_path: mod_dir_path
                        .join(&mod_v1_register_custom_resource_action_payload.src_path),
                },
            );
            log::info!(
                "Executed v1/register-custom-resource action for {}",
                mod_id
            );
        },
        ModActionInfo::ModV1RegisterCustomCharacterActionInfo {
            payload: mod_v1_register_custom_character_action_payload,
        } => unsafe {
            log::info!(
                "Executing v1/register-custom-character-action for {}...",
                mod_id
            );
            CUSTOM_CHARACTERS_INFO.push(
                CustomCharacterInfo {
                    id: mod_v1_register_custom_character_action_payload
                        .id
                        .clone(),
                    name_string_id: mod_v1_register_custom_character_action_payload
                        .name_string_id
                        .clone(),
                    description_string_id: mod_v1_register_custom_character_action_payload
                        .description_string_id
                        .clone(),
                    class: match mod_v1_register_custom_character_action_payload.class {
                        ModV1CharacterClass::Jedi => CustomCharacterClass::Jedi,
                        ModV1CharacterClass::Sith => CustomCharacterClass::Sith,
                        ModV1CharacterClass::RebelResistance => {
                            CustomCharacterClass::RebelResistance
                        },
                        ModV1CharacterClass::BountyHunter => CustomCharacterClass::BountyHunter,
                        ModV1CharacterClass::AstromechDroid => CustomCharacterClass::AstromechDroid,
                        ModV1CharacterClass::ProtocolDroid => CustomCharacterClass::ProtocolDroid,
                        ModV1CharacterClass::Scoundrel => CustomCharacterClass::Scoundrel,
                        ModV1CharacterClass::GalacticEmpire => CustomCharacterClass::GalacticEmpire,
                        ModV1CharacterClass::Scavenger => CustomCharacterClass::Scavenger,
                        ModV1CharacterClass::Civilian => CustomCharacterClass::Civilian,
                    },
                    prefab_resource_path: mod_v1_register_custom_character_action_payload
                        .prefab_resource_path
                        .clone(),
                    preview_prefab_resource_path: mod_v1_register_custom_character_action_payload
                        .preview_prefab_resource_path
                        .clone(),
                },
            );
            log::info!(
                "Executed v1/register-custom-character-action for {}",
                mod_id
            );
        },
    }
}

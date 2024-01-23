use crate::*;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModV1RegisterCustomCharacterActionPayload
{
    pub id: String,
    pub name_string_id: String,
    pub description_string_id: String,
    pub class: ModV1CharacterClass,
    pub prefab_resource_path: String,
    pub preview_prefab_resource_path: String,
}

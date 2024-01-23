use crate::*;

pub struct CustomCharacterInfo
{
    pub id: String,
    pub name_string_id: String,
    pub description_string_id: String,
    pub class: CustomCharacterClass,
    pub prefab_resource_path: String,
    pub preview_prefab_resource_path: String,
}

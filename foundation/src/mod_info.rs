use crate::*;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo
{
    pub name: String,
    pub actions: Vec<ModActionInfo>,
}

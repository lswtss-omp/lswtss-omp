use crate::*;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ModActionInfo
{
    #[serde(rename = "v1/register-custom-resource-action")]
    ModV1RegisterCustomResourceActionInfo
    {
        payload: ModV1RegisterCustomResourceActionPayload,
    },
    #[serde(rename = "v1/register-custom-character-action")]
    ModV1RegisterCustomCharacterActionInfo
    {
        payload: ModV1RegisterCustomCharacterActionPayload,
    },
}

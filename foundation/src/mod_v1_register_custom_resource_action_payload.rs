#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModV1RegisterCustomResourceActionPayload
{
    pub path: String,
    pub src_path: String,
}

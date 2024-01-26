#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ModV1CharacterClass
{
    Jedi,
    Sith,
    RebelResistance,
    BountyHunter,
    AstromechDroid,
    ProtocolDroid,
    Scoundrel,
    GalacticEmpire,
    Scavenger,
    Civilian,
}

use crate::*;

pub struct ModState
{
    pub id: String,
    pub dir_path: std::path::PathBuf,
    pub info: ModInfo,
    pub is_loaded: bool,
}

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ResourceInfo
{
    pub link_name_without_extension: String,
    pub link_name: String,
    pub link_path: String,
    pub canon_name_without_extension: String,
    pub canon_name: String,
    pub canon_path: String,
    pub src_path: PathBuf,
}

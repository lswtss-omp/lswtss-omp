use crate::*;

pub fn register_mods()
{
    let mods_dir_path = std::env::current_dir()
        .unwrap()
        .join("mods");

    for mod_dir_info in mods_dir_path
        .read_dir()
        .unwrap()
    {
        let mod_dir_info = mod_dir_info.unwrap();

        let mod_dir_path = mod_dir_info.path();

        let mod_id = mod_dir_info
            .file_name()
            .into_string()
            .unwrap();

        let mod_info_file_path = mod_dir_path.join("mod.json");

        let mod_info_option = std::fs::read_to_string(&mod_info_file_path)
            .ok()
            .and_then(|mod_info_as_json| serde_json::from_str::<ModInfo>(&mod_info_as_json).ok());

        if let Some(mod_info) = mod_info_option {
            log::info!(
                "Registered {} from {}",
                mod_id,
                mod_info_file_path
                    .to_str()
                    .unwrap(),
            );

            log::info!(
                "mod_info.name: {}",
                mod_info.name
            );

            unsafe {
                MODS_STATE.push(
                    ModState {
                        id: mod_id,
                        dir_path: mod_dir_path,
                        info: mod_info,
                        is_loaded: false,
                    },
                )
            };
        } else {
            log::info!(
                "Couldn't register {} from {}",
                mod_id,
                mod_info_file_path
                    .to_str()
                    .unwrap(),
            );
        }
    }
}

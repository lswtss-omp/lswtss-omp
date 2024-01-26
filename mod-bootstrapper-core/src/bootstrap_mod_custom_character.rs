use std::path::Path;

use crate::*;

fn random_str(len: usize) -> String
{
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let s: String = (0..len)
        .map(
            |_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            },
        )
        .collect();
    s
}

pub fn bootstrap_mod_custom_character(
    mod_dir_path: &Path,
    mod_custom_character_id: &str,
    mod_custom_character_class: &ModV1CharacterClass,
    base_character_prefab_baked_resource_info: &PrefabBakedResourceInfo,
) -> Vec<ModActionInfo>
{
    let mut mod_custom_character_actions_info = Vec::new();

    let mod_custom_character_dir_path = mod_dir_path.join(mod_custom_character_id);

    std::fs::create_dir_all(&mod_custom_character_dir_path).unwrap();

    let mod_custom_character_prefab_resource_link_name_without_extension = random_str(
        base_character_prefab_baked_resource_info
            .base
            .link_name_without_extension
            .len(),
    );

    let mod_custom_character_prefab_resource_canon_name = base_character_prefab_baked_resource_info
        .base
        .canon_name
        .replace(
            &base_character_prefab_baked_resource_info
                .base
                .link_name_without_extension,
            &mod_custom_character_prefab_resource_link_name_without_extension,
        );

    let mod_custom_character_prefab_resource_canon_path = base_character_prefab_baked_resource_info
        .base
        .canon_path
        .replace(
            &base_character_prefab_baked_resource_info
                .base
                .canon_name,
            &mod_custom_character_prefab_resource_canon_name,
        );

    let mod_custom_character_prefab_resource_src_path =
        mod_custom_character_dir_path.join(&mod_custom_character_prefab_resource_canon_name);

    let mut mod_custom_character_prefab_resource_data = std::fs::read(
        &base_character_prefab_baked_resource_info
            .base
            .src_path,
    )
    .unwrap();

    let mod_custom_character_prefab_resource_data_character_guid_entry_info =
        try_get_character_prefab_resource_data_character_guid_entry_info(
            &mod_custom_character_prefab_resource_data,
        )
        .unwrap();

    set_resource_data_guid_entry_value(
        &mut mod_custom_character_prefab_resource_data,
        &mod_custom_character_prefab_resource_data_character_guid_entry_info,
        &get_custom_character_guid(mod_custom_character_id),
    );

    {
        let mod_custom_character_prefab_resource_data_string_entries_info =
            get_resource_data_string_entries_info(&mod_custom_character_prefab_resource_data);

        for mod_custom_character_prefab_resource_data_string_entry in
            mod_custom_character_prefab_resource_data_string_entries_info
        {
            if mod_custom_character_prefab_resource_data_string_entry
                .value
                .to_lowercase()
                == base_character_prefab_baked_resource_info
                    .base
                    .canon_path
            {
                set_resource_data_string_entry_value(
                    &mut mod_custom_character_prefab_resource_data,
                    &mod_custom_character_prefab_resource_data_string_entry,
                    &mod_custom_character_prefab_resource_canon_path,
                );

                break;
            }
        }
    }

    for base_character_prefab_baked_resource_dependency_info in
        &base_character_prefab_baked_resource_info.dependencies
    {
        let mod_custom_character_dependency_resource_link_name_without_extension = random_str(
            base_character_prefab_baked_resource_dependency_info
                .link_name_without_extension
                .len(),
        );

        let mod_custom_character_dependency_resource_link_name =
            base_character_prefab_baked_resource_dependency_info
                .link_name
                .replace(
                    &base_character_prefab_baked_resource_dependency_info
                        .link_name_without_extension,
                    &mod_custom_character_dependency_resource_link_name_without_extension,
                );

        let mod_custom_character_dependency_resource_link_path =
            base_character_prefab_baked_resource_dependency_info
                .link_path
                .replace(
                    &base_character_prefab_baked_resource_dependency_info.link_name,
                    &mod_custom_character_dependency_resource_link_name,
                );

        let mod_custom_character_dependency_resource_canon_name =
            base_character_prefab_baked_resource_dependency_info
                .canon_name
                .replace(
                    &base_character_prefab_baked_resource_dependency_info
                        .link_name_without_extension,
                    &mod_custom_character_dependency_resource_link_name_without_extension,
                );

        let mod_custom_character_dependency_resource_canon_path =
            base_character_prefab_baked_resource_dependency_info
                .canon_path
                .replace(
                    &base_character_prefab_baked_resource_dependency_info.canon_name,
                    &mod_custom_character_dependency_resource_canon_name,
                );

        let mod_custom_character_dependency_resource_src_path = mod_custom_character_dir_path
            .join(&mod_custom_character_dependency_resource_canon_name);

        let mut mod_custom_character_dependency_resource_data =
            std::fs::read(&base_character_prefab_baked_resource_dependency_info.src_path).unwrap();

        let mod_custom_character_dependency_resource_data_string_entries_info =
            get_resource_data_string_entries_info(&mod_custom_character_dependency_resource_data);

        for mod_custom_character_dependency_resource_data_string_entry_info in
            mod_custom_character_dependency_resource_data_string_entries_info
        {
            if mod_custom_character_dependency_resource_data_string_entry_info
                .value
                .to_lowercase()
                == base_character_prefab_baked_resource_dependency_info.canon_path
            {
                println!(
                    "Changing dependency resource canon path from {} to {}",
                    base_character_prefab_baked_resource_dependency_info.canon_path,
                    mod_custom_character_dependency_resource_canon_path
                );

                set_resource_data_string_entry_value(
                    &mut mod_custom_character_dependency_resource_data,
                    &mod_custom_character_dependency_resource_data_string_entry_info,
                    &mod_custom_character_dependency_resource_canon_path,
                );

                break;
            }
        }

        {
            let mod_custom_character_prefab_resource_data_string_entries_info =
                get_resource_data_string_entries_info(&mod_custom_character_prefab_resource_data);

            for mod_custom_character_prefab_resource_data_string_entry in
                mod_custom_character_prefab_resource_data_string_entries_info
            {
                if mod_custom_character_prefab_resource_data_string_entry
                    .value
                    .to_lowercase()
                    == base_character_prefab_baked_resource_dependency_info.link_name
                {
                    println!(
                        "Changing dependency resource link name from {} to {}",
                        base_character_prefab_baked_resource_dependency_info.link_name,
                        mod_custom_character_dependency_resource_link_name
                    );

                    set_resource_data_string_entry_value(
                        &mut mod_custom_character_prefab_resource_data,
                        &mod_custom_character_prefab_resource_data_string_entry,
                        &mod_custom_character_dependency_resource_link_name,
                    );

                    break;
                }
            }
        }

        {
            let mod_custom_character_prefab_resource_data_string_entries_info =
                get_resource_data_string_entries_info(&mod_custom_character_prefab_resource_data);

            for mod_custom_character_prefab_resource_data_string_entry in
                mod_custom_character_prefab_resource_data_string_entries_info
            {
                if mod_custom_character_prefab_resource_data_string_entry
                    .value
                    .to_lowercase()
                    == base_character_prefab_baked_resource_dependency_info.link_path
                {
                    println!(
                        "Changing dependency resource link path from {} to {}",
                        base_character_prefab_baked_resource_dependency_info.link_path,
                        mod_custom_character_dependency_resource_link_path
                    );

                    set_resource_data_string_entry_value(
                        &mut mod_custom_character_prefab_resource_data,
                        &mod_custom_character_prefab_resource_data_string_entry,
                        &mod_custom_character_dependency_resource_link_path,
                    );

                    break;
                }
            }
        }

        std::fs::write(
            mod_custom_character_dependency_resource_src_path,
            mod_custom_character_dependency_resource_data,
        )
        .unwrap();

        mod_custom_character_actions_info.push(
            ModActionInfo::ModV1RegisterCustomResourceActionInfo {
                payload: ModV1RegisterCustomResourceActionPayload {
                    path: mod_custom_character_dependency_resource_canon_path,
                    src_path: format!(
                        "{}/{}",
                        mod_custom_character_id,
                        mod_custom_character_dependency_resource_canon_name
                    ),
                },
            },
        );
    }

    std::fs::write(
        mod_custom_character_prefab_resource_src_path,
        mod_custom_character_prefab_resource_data,
    )
    .unwrap();

    mod_custom_character_actions_info.push(
        ModActionInfo::ModV1RegisterCustomResourceActionInfo {
            payload: ModV1RegisterCustomResourceActionPayload {
                path: mod_custom_character_prefab_resource_canon_path.clone(),
                src_path: format!(
                    "{}/{}",
                    mod_custom_character_id, mod_custom_character_prefab_resource_canon_name
                ),
            },
        },
    );

    mod_custom_character_actions_info.push(
        ModActionInfo::ModV1RegisterCustomCharacterActionInfo {
            payload: ModV1RegisterCustomCharacterActionPayload {
                id: mod_custom_character_id.to_string(),
                name_string_id: mod_custom_character_id.to_string(),
                description_string_id: mod_custom_character_id.to_string(),
                class: mod_custom_character_class.clone(),
                prefab_resource_path: mod_custom_character_prefab_resource_canon_path,
                preview_prefab_resource_path: "chars/minifig/custom/custom_a.scene_baked"
                    .to_string(),
            },
        },
    );

    return mod_custom_character_actions_info;
}

use std::path::Path;

use crate::*;

pub fn get_prefab_baked_resource_info(
    resources_dir_path: &Path,
    prefab_resource_src_absolute_path: &Path,
) -> PrefabBakedResourceInfo
{
    let prefab_baked_resource_canon_path = prefab_resource_src_absolute_path
        .strip_prefix(resources_dir_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .replace(
            '\\', "/",
        );

    let prefab_baked_resource_canon_name = prefab_baked_resource_canon_path
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let prefab_baked_resource_link_path = prefab_baked_resource_canon_path.clone();

    let prefab_baked_resource_link_name = prefab_baked_resource_link_path
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let mut prefab_baked_resource_info = PrefabBakedResourceInfo {
        base: ResourceInfo {
            link_name_without_extension: prefab_baked_resource_link_name
                .split('.')
                .next()
                .unwrap()
                .to_string(),
            link_name: prefab_baked_resource_link_name,
            link_path: prefab_baked_resource_link_path,
            canon_name_without_extension: prefab_baked_resource_canon_name
                .split('.')
                .next()
                .unwrap()
                .to_string(),
            canon_name: prefab_baked_resource_canon_name,
            canon_path: prefab_baked_resource_canon_path,
            src_path: prefab_resource_src_absolute_path.to_path_buf(),
        },
        dependencies: Vec::new(),
    };

    let prefab_baked_resource_data = std::fs::read(prefab_resource_src_absolute_path).unwrap();

    let prefab_baked_resource_data_string_entries_info =
        get_resource_data_string_entries_info(&prefab_baked_resource_data);

    for prefab_baked_resource_data_string_entry_info in
        prefab_baked_resource_data_string_entries_info
    {
        let prefab_baked_resource_dependency_link_path =
            prefab_baked_resource_data_string_entry_info
                .value
                .to_lowercase()
                .replace(
                    "_dx11.texture",
                    ".texture",
                )
                .replace(
                    "_dx11.gsc",
                    ".gsc",
                );

        let prefab_baked_resource_dependency_canon_path =
            prefab_baked_resource_dependency_link_path.replace(
                ".texture",
                "_dx11.texture",
            ).replace(
                ".gsc",
                "_dx11.gsc",
            );

        let prefab_baked_resource_dependency_src_path =
            resources_dir_path.join(&prefab_baked_resource_dependency_canon_path);

        if prefab_baked_resource_dependency_canon_path.starts_with("chars") {
            println!(
                "checking {:?}",
                prefab_baked_resource_dependency_src_path
            );
        }

        if prefab_baked_resource_dependency_src_path.exists() {
            if let Some(prefab_baked_resource_dependency_extension) =
                prefab_baked_resource_dependency_src_path.extension()
            {
                if prefab_baked_resource_dependency_extension == "texture"
                    || prefab_baked_resource_dependency_extension == "gsc"
                {
                    let prefab_baked_resource_dependency_link_name =
                        prefab_baked_resource_dependency_link_path
                            .split('/')
                            .last()
                            .unwrap()
                            .to_string();

                    let prefab_baked_resource_dependency_canon_name =
                        prefab_baked_resource_dependency_canon_path
                            .split('/')
                            .last()
                            .unwrap()
                            .to_string();

                    prefab_baked_resource_info
                        .dependencies
                        .push(
                            ResourceInfo {
                                link_name_without_extension:
                                    prefab_baked_resource_dependency_link_name
                                        .split('.')
                                        .next()
                                        .unwrap()
                                        .to_string(),
                                link_name: prefab_baked_resource_dependency_link_name,
                                link_path: prefab_baked_resource_dependency_link_path,
                                canon_name_without_extension:
                                    prefab_baked_resource_dependency_canon_name
                                        .split('.')
                                        .next()
                                        .unwrap()
                                        .to_string(),
                                canon_name: prefab_baked_resource_dependency_canon_name,
                                canon_path: prefab_baked_resource_dependency_canon_path,
                                src_path: prefab_baked_resource_dependency_src_path,
                            },
                        );
                }
            }
        }
    }

    return prefab_baked_resource_info;
}

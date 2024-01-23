use std::path::Path;

use crate::*;

pub fn link_custom_resources()
{
    let current_dir_path = std::env::current_dir().unwrap();

    unsafe {
        for custom_resource_info in CUSTOM_RESOURCES_INFO.iter() {
            log::info!(
                "Linking custom resource {:?}...",
                custom_resource_info.path
            );

            let custom_resource_abs_path = current_dir_path.join(&custom_resource_info.path);
            let custom_resources_src_abs_path =
                current_dir_path.join(&custom_resource_info.src_path);

            let custom_resource_dir_path = Path::new(&custom_resource_abs_path).parent();

            std::fs::create_dir_all(current_dir_path.join(custom_resource_dir_path.unwrap()))
                .unwrap();

            if custom_resource_abs_path.exists() {
                std::fs::remove_file(&custom_resource_abs_path).unwrap();
            }

            std::fs::hard_link(
                &custom_resources_src_abs_path,
                &custom_resource_abs_path,
            )
            .unwrap();

            DISK_RESOURCES_INFO.push(
                DiskResourceInfo {
                    canon_path: get_resource_canon_path(&custom_resource_info.path),
                },
            );

            log::info!(
                "Linked custom resource {:?}",
                custom_resource_info.path
            );
        }
    }
}

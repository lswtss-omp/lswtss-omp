use crate::*;

pub fn fetch_is_disk_resource_registered(resource_canon_path: &str) -> bool
{
    unsafe {
        for disk_resource_info in DISK_RESOURCES_INFO.iter() {
            if disk_resource_info.canon_path == resource_canon_path {
                return true;
            }
        }
    }

    return false;
}

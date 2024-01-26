use crate::*;

pub fn get_resource_data_string_entries_info(
    resource_data: &ResourceData
) -> Vec<ResourceDataStringEntryInfo>
{
    let mut resource_data_string_entries_info = Vec::new();

    let mut offset = 0;

    while offset < resource_data.len() - 4 {
        if let Some(resource_data_string_entry_info) = try_get_resource_data_string_entry(
            resource_data,
            offset,
        ) {
            resource_data_string_entries_info.push(resource_data_string_entry_info);
        }

        offset += 1;
    }

    return resource_data_string_entries_info;
}

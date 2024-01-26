use std::convert::TryInto;

use crate::*;

pub fn try_get_character_prefab_resource_data_character_guid_entry_info(
    resource_data: &ResourceData
) -> Option<ResourceDataGuidEntryInfo>
{
    for i in 0..resource_data.len() - 15 {
        // Character GUID is preceeded by "Characters" string (prefixed with length)

        if resource_data[i] == 0x0B
            && resource_data[i + 1] == 0x00
            && resource_data[i + 2] == 0x00
            && resource_data[i + 3] == 0x00
            && resource_data[i + 4] == 0x43
            && resource_data[i + 5] == 0x68
            && resource_data[i + 6] == 0x61
            && resource_data[i + 7] == 0x72
            && resource_data[i + 8] == 0x61
            && resource_data[i + 9] == 0x63
            && resource_data[i + 10] == 0x74
            && resource_data[i + 11] == 0x65
            && resource_data[i + 12] == 0x72
            && resource_data[i + 13] == 0x73
            && resource_data[i + 14] == 0x00
        {
            let resource_data_character_guid_entry_offset = i + 15;

            if let Ok(resource_data_character_guid_entry_value) = resource_data
                [resource_data_character_guid_entry_offset
                    ..resource_data_character_guid_entry_offset + 16]
                .try_into()
                .map(u128::from_le_bytes)
            {
                return Some(
                    ResourceDataGuidEntryInfo {
                        value: resource_data_character_guid_entry_value,
                        offset: resource_data_character_guid_entry_offset,
                    },
                );
            }
        }
    }

    return None;
}

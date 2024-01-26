use crate::*;

pub fn set_resource_data_guid_entry_value(
    resource_data: &mut ResourceData,
    resource_data_guid_entry_info: &ResourceDataGuidEntryInfo,
    resource_data_guid_entry_new_value: &u128,
)
{
    resource_data[resource_data_guid_entry_info.offset..resource_data_guid_entry_info.offset + 16]
        .copy_from_slice(&resource_data_guid_entry_new_value.to_le_bytes());
}

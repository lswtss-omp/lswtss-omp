use crate::*;

pub fn set_resource_data_string_entry_value(
    resource_data: &mut ResourceData,
    resource_data_string_entry_info: &ResourceDataStringEntryInfo,
    resource_data_string_entry_new_value: &str,
)
{
    let resource_data_string_entry_new_value_length = resource_data_string_entry_new_value.len();

    if resource_data_string_entry_new_value_length
        != resource_data_string_entry_info
            .value
            .len()
    {
        panic!();
    }

    let resource_data_string_entry_new_value_as_ascii = resource_data_string_entry_new_value
        .as_ascii()
        .unwrap();

    for i in 0..resource_data_string_entry_info
        .value
        .len()
    {
        resource_data[resource_data_string_entry_info.offset + i] =
            resource_data_string_entry_new_value_as_ascii[i].to_u8();
    }
}

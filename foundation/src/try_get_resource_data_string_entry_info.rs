use crate::*;

pub fn try_get_resource_data_string_entry(
    resource_data: &ResourceData,
    resource_data_string_entry_offset: usize,
) -> Option<ResourceDataStringEntryInfo>
{
    let mut resource_data_string_entry_value = String::new();

    const RESOURCE_DATA_STRING_ENTRY_VALUE_MIN_LENGTH: usize = 6;
    const RESOURCE_DATA_STRING_ENTRY_VALUE_MAX_LENGTH: usize = 256;

    let mut j = 0;

    loop {
        if j > RESOURCE_DATA_STRING_ENTRY_VALUE_MAX_LENGTH {
            return None;
        }

        let i = resource_data_string_entry_offset + j;

        if i >= resource_data.len() {
            return None;
        }

        if resource_data[i] == 0x00 {
            break;
        }

        if let Some(resource_data_string_entry_value_char) =
            std::ascii::Char::from_u8(resource_data[i])
        {
            resource_data_string_entry_value.push(resource_data_string_entry_value_char.to_char());
        } else {
            return None;
        }

        j += 1;
    }

    if j < RESOURCE_DATA_STRING_ENTRY_VALUE_MIN_LENGTH {
        return None;
    }

    return Some(
        ResourceDataStringEntryInfo {
            value: resource_data_string_entry_value,
            offset: resource_data_string_entry_offset,
        },
    );
}

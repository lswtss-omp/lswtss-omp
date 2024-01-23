use crate::*;

pub fn get_raw_nu_guid_from_custom_character_id(custom_character_id: &str) -> RawNuGuid
{
    unsafe {
        return std::mem::transmute(
            uuid::Uuid::new_v5(
                &uuid::Uuid::NAMESPACE_OID,
                custom_character_id.as_bytes(),
            )
            .as_bytes()
            .to_owned(),
        );
    }
}

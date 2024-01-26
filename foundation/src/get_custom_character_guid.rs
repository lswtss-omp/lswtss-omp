pub fn get_custom_character_guid(custom_character_id: &str) -> u128
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

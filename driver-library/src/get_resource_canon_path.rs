pub fn get_resource_canon_path(resource_path: &str) -> String
{
    return resource_path
        .to_lowercase()
        .replace(
            '\\', "/",
        );
}

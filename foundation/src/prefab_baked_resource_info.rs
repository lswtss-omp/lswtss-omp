use crate::*;

#[derive(Clone, Debug)]

pub struct PrefabBakedResourceInfo
{
    pub base: ResourceInfo,
    pub dependencies: Vec<ResourceInfo>,
}

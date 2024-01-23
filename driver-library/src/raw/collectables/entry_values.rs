use crate::*;

#[repr(C)]
pub struct RawCollectablesEntryValues
{
    pub values: RawNuVector<RawCollectablesTableValue>,
}

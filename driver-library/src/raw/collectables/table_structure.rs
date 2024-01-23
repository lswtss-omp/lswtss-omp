use std::ffi::c_void;

use crate::*;

#[repr(C)]
pub struct RawCollectablesTableStructure
{
    pub unknown_field_1: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub columns: RawNuVector<c_void>,
    pub entries: RawNuVector<RawCollectablesTableEntry>,
}

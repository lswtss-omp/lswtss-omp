use std::ffi::c_char;

use crate::*;

#[repr(C)]
pub struct RawCollectablesTableEntry
{
    pub name_1: *mut c_char,
    pub name_2: *mut c_char,
    pub guid: RawNuGuid,
    pub dlc_version: RawNuGuid,
}

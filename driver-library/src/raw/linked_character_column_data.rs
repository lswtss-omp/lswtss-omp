use std::ffi::c_ulonglong;
use std::ffi::c_void;

use crate::*;

#[repr(C)]
pub struct RawLinkedCharacterColumnData
{
    pub base: RawCollectablesCustomDataVal,
    pub unknown_field_1: *mut c_void,
    pub unknown_field_2: c_ulonglong, // set by game when adding new collectables::Table entry
    pub unknown_field_3: RawNuVector<c_void>,
    pub unknown_field_4: *mut c_void, // 0x0000000000000000
}

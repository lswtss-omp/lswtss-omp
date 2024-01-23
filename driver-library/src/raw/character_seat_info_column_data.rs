use std::ffi::c_void;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCharacterSeatInfoColumnData
{
    pub base: RawCollectablesCustomDataVal,
    pub unknown_field_1: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub unknown_field_4: *mut c_void,
    pub unknown_field_5: *mut c_void,
    pub unknown_field_6: *mut c_void,
}

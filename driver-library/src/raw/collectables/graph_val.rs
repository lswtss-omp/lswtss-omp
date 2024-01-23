use std::ffi::c_void;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCollectablesGraphVal
{
    pub vtable: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub value: RawNttResourcesNttResourceHandle,
}

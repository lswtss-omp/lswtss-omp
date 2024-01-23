use std::ffi::c_uchar;
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCollectablesBoolVal
{
    pub vtable: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub value: c_uchar,
}

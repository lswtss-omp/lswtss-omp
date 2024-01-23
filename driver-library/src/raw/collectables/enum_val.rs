use std::ffi::c_int;
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCollectablesEnumVal
{
    pub vtable: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub value: c_int,
    pub unknown_field_5: c_int,
    pub unknown_field_6: *mut c_void,
    pub unknown_field_7: *mut c_void,
}

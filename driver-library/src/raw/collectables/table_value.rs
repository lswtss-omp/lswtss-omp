use std::ffi::c_void;

pub struct RawCollectablesTableValue
{
    pub value: *mut c_void,
    pub unknown_field_2: *mut c_void,
}

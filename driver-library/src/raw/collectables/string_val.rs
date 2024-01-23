use std::ffi::c_char;
use std::ffi::c_ushort;
use std::ffi::c_void;
use std::ffi::CString;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCollectablesStringVal
{
    pub vtable: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub value: *mut c_char,
    pub value_length: c_ushort,
    pub value_length2: c_ushort,
}

impl RawCollectablesStringVal
{
    pub fn set_value_as_str(
        &mut self,
        value: &str,
    )
    {
        self.value = CString::new(value)
            .unwrap()
            .into_raw();
        self.value_length = (value.len() + 1) as c_ushort;
        self.value_length2 = (value.len() + 1) as c_ushort;
    }
}

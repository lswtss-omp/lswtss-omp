use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ffi::c_void;
use std::ffi::CString;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawNttResourcesNttResourceHandle
{
    pub vtable: *mut c_void,
    pub unknown_field_2: *mut c_void,
    pub unknown_field_3: *mut c_void,
    pub resource_path: *mut c_char,
    pub unknown_field_5: c_uchar,
    pub unknown_field_6: c_uchar,
    pub resource_path_length: c_uchar,
    pub unknown_field_8: c_uchar,
    pub unknown_field_9: c_int,
    pub unknown_field_10: *mut c_void,
    pub unknown_field_11_ptr: *mut c_void,
    pub unknown_field_12: *mut c_void,
    pub unknown_field_13: *mut c_void,
    pub unknown_field_14: *mut c_void,
    pub unknown_field_15_ptr: *mut c_void,
    pub unknown_field_16_ptr: *mut c_void,
    pub unknown_field_17_ptr: *mut c_void,
    pub unknown_field_18: *mut c_void,
    pub unknown_field_19: *mut c_void,
}

impl RawNttResourcesNttResourceHandle
{
    pub fn set_resource_path_as_str(
        &mut self,
        resource_path: &str,
    )
    {
        self.resource_path = CString::new(resource_path)
            .unwrap()
            .into_raw();
        self.resource_path_length = (resource_path.len() + 1) as c_uchar;
        self.unknown_field_5 = 0;
        self.unknown_field_6 = 2;
    }
}

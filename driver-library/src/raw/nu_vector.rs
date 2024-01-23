#[repr(C)]
pub struct RawNuVector<T>
{
    pub unknown_field_1_ptr: *mut T,
    pub unknown_field_2_ptr: *mut T,
    pub unknown_field_3_ptr: *mut T,
}

impl<T> RawNuVector<T>
{
    pub fn get_handle_at_index(
        &self,
        index: usize,
    ) -> *mut T
    {
        unsafe {
            self.unknown_field_1_ptr
                .add(index)
        }
    }
}

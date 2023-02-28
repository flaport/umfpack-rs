use super::di::umfpack_di_free_numeric;
use super::zi::umfpack_zi_free_numeric;
use std::ffi::c_void;

pub struct Numeric {
    pub data: *mut c_void,
}

impl Numeric {
    pub fn new() -> Self {
        return Self {
            data: std::ptr::null_mut() as *mut c_void,
        };
    }
}

impl Drop for Numeric {
    fn drop(&mut self) {
        umfpack_di_free_numeric(self);
        umfpack_zi_free_numeric(self);
    }
}

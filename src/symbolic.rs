use std::ffi::c_void;
use super::di::umfpack_di_free_symbolic;
use super::zi::umfpack_zi_free_symbolic;

pub struct Symbolic {
    pub data: *mut c_void,
}

impl Symbolic {
    pub fn new() -> Self {
        return Self {
            data: std::ptr::null_mut() as *mut c_void,
        };
    }
}

impl Drop for Symbolic {
    fn drop(&mut self) {
        umfpack_di_free_symbolic(self);
        umfpack_zi_free_symbolic(self);
    }
}

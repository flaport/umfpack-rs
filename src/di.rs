use super::control::Control;
use super::info::Info;
use super::sys::UMFPACK;
use std::ffi::c_void;

mod c {
    use std::ffi::c_void;

    extern "C" {
        pub fn umfpack_di_symbolic(
            n: i32,
            m: i32,
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            Symbolic: *mut *mut c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
        pub fn umfpack_di_numeric(
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            Symbolic: *const c_void,
            Numeric: *mut *mut c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
        pub fn umfpack_di_free_symbolic(Symbolic: *mut *mut c_void);
        pub fn umfpack_di_free_numeric(Numeric: *mut *mut c_void);
        pub fn umfpack_di_solve(
            sys: i32,
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            X: *mut f64,
            B: *const f64,
            Numeric: *const c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
    }
}

pub struct Symbolic {
    data: *mut c_void,
}

impl Symbolic {
    pub fn new() -> Self {
        return Self {
            data: std::ptr::null_mut() as *mut c_void,
        };
    }
}

#[allow(non_snake_case)]
pub fn umfpack_di_symbolic(
    n: i32,
    m: i32,
    Ap: &[i32],
    Ai: &[i32],
    Ax: &[f64],
    symbolic: &mut Symbolic,
    control: &Control,
    info: &mut Info,
) -> i32 {
    unsafe {
        c::umfpack_di_symbolic(
            n,
            m,
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            &mut symbolic.data as *mut *mut c_void,
            control.data().as_ptr(),
            info.data.as_mut_ptr(),
        )
    }
}

pub struct Numeric {
    data: *mut c_void,
}

impl Numeric {
    pub fn new() -> Self {
        return Self {
            data: std::ptr::null_mut() as *mut c_void,
        };
    }
}

#[allow(non_snake_case)]
pub fn umfpack_di_numeric(
    Ap: &[i32],
    Ai: &[i32],
    Ax: &[f64],
    symbolic: &Symbolic,
    numeric: &mut Numeric,
    control: &Control,
    info: &mut Info,
) -> i32 {
    unsafe {
        c::umfpack_di_numeric(
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            symbolic.data,
            &mut numeric.data as *mut *mut c_void,
            control.data().as_ptr(),
            info.data.as_mut_ptr(),
        )
    }
}

pub fn umfpack_di_free_symbolic(symbolic: &mut Symbolic) {
    unsafe { c::umfpack_di_free_symbolic(&mut symbolic.data as *mut *mut c_void) }
}

pub fn umfpack_di_free_numeric(numeric: &mut Numeric) {
    unsafe { c::umfpack_di_free_numeric(&mut numeric.data as *mut *mut c_void) }
}

impl Drop for Symbolic {
    fn drop(&mut self) {
        umfpack_di_free_symbolic(self);
    }
}

impl Drop for Numeric {
    fn drop(&mut self) {
        umfpack_di_free_numeric(self);
    }
}

#[allow(non_snake_case)]
pub fn umfpack_di_solve(
    sys: UMFPACK,
    Ap: &[i32],
    Ai: &[i32],
    Ax: &[f64],
    X: &mut [f64],
    B: &[f64],
    numeric: &Numeric,
    control: &Control,
    info: &mut Info,
) -> i32 {
    unsafe {
        c::umfpack_di_solve(
            sys.to_int(),
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            X.as_mut_ptr(),
            B.as_ptr(),
            numeric.data,
            control.data().as_ptr(),
            info.data.as_mut_ptr(),
        )
    }
}

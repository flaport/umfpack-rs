use super::control::Control;
use super::info::Info;
use super::numeric::Numeric;
use super::symbolic::Symbolic;
use super::sys::UMFPACK;
use num_complex::Complex64;
use std::ffi::c_void;
use std::ptr;

mod c {
    use std::ffi::c_void;

    extern "C" {
        pub fn umfpack_zi_symbolic(
            n: i32,
            m: i32,
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            Az: *const f64,
            Symbolic: *mut *mut c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
        pub fn umfpack_zi_numeric(
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            Az: *const f64,
            Symbolic: *const c_void,
            Numeric: *mut *mut c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
        pub fn umfpack_zi_free_symbolic(Symbolic: *mut *mut c_void);
        pub fn umfpack_zi_free_numeric(Numeric: *mut *mut c_void);
        pub fn umfpack_zi_solve(
            sys: i32,
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            Az: *const f64,
            Xx: *mut f64,
            Xz: *mut f64,
            Bx: *const f64,
            Bz: *const f64,
            Numeric: *const c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
    }
}

#[allow(non_snake_case)]
pub fn umfpack_zi_symbolic(
    n: i32,
    m: i32,
    Ap: &[i32],
    Ai: &[i32],
    Az: &[Complex64],
    symbolic: &mut Symbolic,
    control: Option<&Control>,
    info: Option<&mut Info>,
) -> i32 {
    unsafe {
        c::umfpack_zi_symbolic(
            n,
            m,
            Ap.as_ptr(),
            Ai.as_ptr(),
            Az.as_ptr() as *const f64,
            ptr::null(),
            &mut symbolic.data as *mut *mut c_void,
            match control {
                None => ptr::null(),
                Some(c) => c.data().as_ptr(),
            },
            match info {
                None => ptr::null_mut(),
                Some(i) => i.data.as_mut_ptr(),
            },
        )
    }
}

#[allow(non_snake_case)]
pub fn umfpack_zi_numeric(
    Ap: &[i32],
    Ai: &[i32],
    Az: &[Complex64],
    symbolic: &Symbolic,
    numeric: &mut Numeric,
    control: Option<&Control>,
    info: Option<&mut Info>,
) -> i32 {
    unsafe {
        c::umfpack_zi_numeric(
            Ap.as_ptr(),
            Ai.as_ptr(),
            Az.as_ptr() as *const f64,
            ptr::null(),
            symbolic.data,
            &mut numeric.data as *mut *mut c_void,
            match control {
                None => ptr::null(),
                Some(c) => c.data().as_ptr(),
            },
            match info {
                None => ptr::null_mut(),
                Some(i) => i.data.as_mut_ptr(),
            },
        )
    }
}

pub fn umfpack_zi_free_symbolic(symbolic: &mut Symbolic) {
    unsafe { c::umfpack_zi_free_symbolic(&mut symbolic.data as *mut *mut c_void) }
}

pub fn umfpack_zi_free_numeric(numeric: &mut Numeric) {
    unsafe { c::umfpack_zi_free_numeric(&mut numeric.data as *mut *mut c_void) }
}

#[allow(non_snake_case)]
pub fn umfpack_zi_solve(
    sys: UMFPACK,
    Ap: &[i32],
    Ai: &[i32],
    Az: &[Complex64],
    Xz: &mut [Complex64],
    Bz: &[Complex64],
    numeric: &Numeric,
    control: Option<&Control>,
    info: Option<&mut Info>,
) -> i32 {
    unsafe {
        c::umfpack_zi_solve(
            sys.to_int(),
            Ap.as_ptr(),
            Ai.as_ptr(),
            Az.as_ptr() as *const f64,
            ptr::null(),
            Xz.as_mut_ptr() as *mut f64,
            ptr::null_mut(),
            Bz.as_ptr() as *const f64,
            ptr::null(),
            numeric.data,
            match control {
                None => ptr::null(),
                Some(c) => c.data().as_ptr(),
            },
            match info {
                None => ptr::null_mut(),
                Some(i) => i.data.as_mut_ptr(),
            },
        )
    }
}

use libc::strlen;
use std::ffi::c_void;
use std::slice;

mod c {
    use std::ffi::c_void;

    extern "C" {
        pub fn example();
        pub fn SuiteSparse_BLAS_library() -> *const i8;
        pub fn solve(
            n: i32,
            Ap: *const i32,
            Ai: *const i32,
            Ax: *const f64,
            b: *const f64,
            Symbolic: *mut c_void,
        );

        #[allow(dead_code)]
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
    }
}

pub fn example() {
    unsafe { c::example() }
}

#[allow(non_snake_case)]
pub fn solve(n: i32, Ap: &[i32], Ai: &[i32], Ax: &[f64], b: &[f64], symbolic: &mut Symbolic) {
    unsafe {
        c::solve(
            n,
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            b.as_ptr(),
            symbolic._data,
        )
    }
}

#[allow(non_snake_case)]
pub fn SuiteSparse_BLAS_library() -> String {
    let ptr = unsafe { c::SuiteSparse_BLAS_library() };
    let len = unsafe { strlen(ptr) };
    let vec = unsafe { slice::from_raw_parts(ptr, len) };
    let string = String::from_utf8(vec.iter().map(|i| *i as u8).collect()).unwrap();
    return string;
}

pub struct Symbolic {
    _data: *mut c_void,
}

impl Symbolic {
    pub fn new() -> Self {
        return Self { _data: std::ptr::null_mut() as *mut c_void };
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
    //Control: None,
    //Info: None,
) -> i32 {
    unsafe {
        let null: *mut f64 = std::ptr::null_mut();
        c::umfpack_di_symbolic(
            n,
            m,
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            &mut symbolic._data as *mut *mut c_void,
            null,
            null,
        )
    }
}

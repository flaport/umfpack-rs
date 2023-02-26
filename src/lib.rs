use libc::strlen;
use std::slice;

mod c {
    extern "C" {
        pub fn example();
        pub fn SuiteSparse_BLAS_library() -> *const i8;
        pub fn solve(n: i32, Ap: *const i32, Ai: *const i32, Ax: *const f64, b: *const f64);
    }
}

pub fn example() {
    unsafe { c::example() }
}

#[allow(non_snake_case)]
pub fn solve(n: i32, Ap: &[i32], Ai: &[i32], Ax: &[f64], b: &[f64]) {
    unsafe { c::solve(n, Ap.as_ptr(), Ai.as_ptr(), Ax.as_ptr(), b.as_ptr()) }
}

#[allow(non_snake_case)]
pub fn SuiteSparse_BLAS_library() -> String {
    let ptr = unsafe { c::SuiteSparse_BLAS_library() };
    let len = unsafe { strlen(ptr) };
    let vec = unsafe { slice::from_raw_parts(ptr, len) };
    let string = String::from_utf8(vec.iter().map(|i| *i as u8).collect()).unwrap();
    return string;
}

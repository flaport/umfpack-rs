use libc::strlen;
use std::ffi::c_void;
use std::slice;

mod c {
    use std::ffi::c_void;

    extern "C" {
        pub fn example();
        pub fn SuiteSparse_BLAS_library() -> *const i8;
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
            Symbolic: *mut c_void,
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
            Numeric: *mut c_void,
            Control: *const f64,
            Info: *mut f64,
        ) -> i32;
    }
}

pub fn example() {
    unsafe { c::example() }
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
        return Self {
            _data: std::ptr::null_mut() as *mut c_void,
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
    //Control: None,
    //Info: None,
) -> i32 {
    let control: *mut f64 = std::ptr::null_mut();
    let info: *mut f64 = std::ptr::null_mut();
    unsafe {
        c::umfpack_di_symbolic(
            n,
            m,
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            &mut symbolic._data as *mut *mut c_void,
            control,
            info,
        )
    }
}

pub struct Numeric {
    _data: *mut c_void,
}

impl Numeric {
    pub fn new() -> Self {
        return Self {
            _data: std::ptr::null_mut() as *mut c_void,
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
    //Control: None,
    //Info: None,
) -> i32 {
    let control: *mut f64 = std::ptr::null_mut();
    let info: *mut f64 = std::ptr::null_mut();
    unsafe {
        c::umfpack_di_numeric(
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            symbolic._data,
            &mut numeric._data as *mut *mut c_void,
            control,
            info,
        )
    }
}

pub fn umfpack_di_free_symbolic(symbolic: &mut Symbolic) {
    unsafe { c::umfpack_di_free_symbolic(&mut symbolic._data as *mut *mut c_void) }
}

pub fn umfpack_di_free_numeric(numeric: &mut Numeric) {
    unsafe { c::umfpack_di_free_numeric(&mut numeric._data as *mut *mut c_void) }
}

#[allow(non_camel_case_types)]
pub enum UMFPACK {
    A,     /* Ax=b    */
    At,    /* A'x=b   */
    Aat,   /* A.'x=b  */
    Pt_L,  /* P'Lx=b  */
    L,     /* Lx=b    */
    Lt_P,  /* L'Px=b  */
    Lat_P, /* L.'Px=b */
    Lt,    /* L'x=b   */
    Lat,   /* L.'x=b  */
    U_Qt,  /* UQ'x=b  */
    U,     /* Ux=b    */
    Q_Ut,  /* QU'x=b  */
    Q_Uat, /* QU.'x=b */
    Ut,    /* U'x=b   */
    Uat,   /* U.'x=b  */
}

impl UMFPACK {
    pub fn to_int(&self) -> i32 {
        match self {
            UMFPACK::A => 0,
            UMFPACK::At => 1,
            UMFPACK::Aat => 2,
            UMFPACK::Pt_L => 3,
            UMFPACK::L => 4,
            UMFPACK::Lt_P => 5,
            UMFPACK::Lat_P => 6,
            UMFPACK::Lt => 7,
            UMFPACK::Lat => 8,
            UMFPACK::U_Qt => 9,
            UMFPACK::U => 10,
            UMFPACK::Q_Ut => 11,
            UMFPACK::Q_Uat => 12,
            UMFPACK::Ut => 13,
            UMFPACK::Uat => 14,
        }
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
    // Control,
    // Info,
) -> i32 {
    let control: *mut f64 = std::ptr::null_mut();
    let info: *mut f64 = std::ptr::null_mut();
    unsafe {
        c::umfpack_di_solve(
            sys.to_int(),
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            X.as_mut_ptr(),
            B.as_ptr(),
            numeric._data,
            control,
            info,
        )
    }
}

impl Drop for Symbolic {
    fn drop(&mut self){
        umfpack_di_free_symbolic(self);
    }
}

impl Drop for Numeric {
    fn drop(&mut self){
        umfpack_di_free_numeric(self);
    }
}

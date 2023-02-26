use array_init::array_init;
use libc::strlen;
use std::ffi::c_void;
use std::slice;

mod c {
    use std::ffi::c_void;

    extern "C" {
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
            &mut symbolic._data as *mut *mut c_void,
            control._data().as_ptr(),
            info._data.as_mut_ptr(),
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
    control: &Control,
    info: &mut Info,
) -> i32 {
    unsafe {
        c::umfpack_di_numeric(
            Ap.as_ptr(),
            Ai.as_ptr(),
            Ax.as_ptr(),
            symbolic._data,
            &mut numeric._data as *mut *mut c_void,
            control._data().as_ptr(),
            info._data.as_mut_ptr(),
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
            numeric._data,
            control._data().as_ptr(),
            info._data.as_mut_ptr(),
        )
    }
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

pub struct Control {
    pub prl: i32,
    pub dense_row: f64,
    pub dense_col: f64,
    pub pivot_tolerance: f64,
    pub block_size: i32,
    pub strategy: Strategy,
    pub alloc_init: f64,
    pub irstep: i32,
    pub compiled_with_blas: bool,
    pub thresh_sym: f64,
    pub ordering: Ordering,
    pub singletons: bool,
    pub thresh_nnzdiag: f64,
    pub fixq: i32,
    pub amd_dense: f64,
    pub sym_pivot_tolerance: f64,
    pub scale: Scale,
    pub front_alloc_init: f64,
    pub droptol: i32,
    pub aggressive: bool,
}

impl Control {
    pub fn new() -> Self {
        return Self {
            prl: 1,
            dense_row: 0.2,
            dense_col: 0.2,
            pivot_tolerance: 0.1,
            block_size: 32,
            strategy: Strategy::AUTO,
            alloc_init: 0.5,
            irstep: 2,
            compiled_with_blas: true,
            thresh_sym: 0.3,
            ordering: Ordering::AMD,
            singletons: false,
            thresh_nnzdiag: 0.9,
            fixq: 0,
            amd_dense: 10.0,
            sym_pivot_tolerance: 0.001,
            scale: Scale::SUM,
            front_alloc_init: 0.5,
            droptol: 0,
            aggressive: false,
        };
    }
    fn _data(&self) -> [f64; 20] {
        let mut data: [f64; 20] = array_init(|_| 0.0);
        data[0] = self.prl as f64;
        data[1] = self.dense_row as f64;
        data[2] = self.dense_col as f64;
        data[3] = self.pivot_tolerance as f64;
        data[4] = self.block_size as f64;
        data[5] = self.strategy.to_int() as f64;
        data[6] = self.alloc_init as f64;
        data[7] = self.irstep as f64;
        data[8] = self.compiled_with_blas as i32 as f64;
        data[9] = self.thresh_sym as f64;
        data[10] = self.ordering.to_int() as f64;
        data[11] = self.singletons as i32 as f64;
        data[12] = self.thresh_nnzdiag as f64;
        data[13] = self.fixq as f64;
        data[14] = self.amd_dense as f64;
        data[15] = self.sym_pivot_tolerance as f64;
        data[16] = self.scale.to_int() as f64;
        data[17] = self.front_alloc_init as f64;
        data[18] = self.droptol as f64;
        data[19] = self.aggressive as i32 as f64;
        return data;
    }
}

#[allow(non_camel_case_types)]
pub enum Strategy {
    THRESH_SYM,     /* symmetry threshold */
    THRESH_NNZDIAG, /* nnz(diag(A)) threshold */
    AUTO,           /* use sym. or unsym. strategy */
    UNSYMMETRIC,    /* COLAMD(A) coletree postorder */
    OBSOLETE,       /* 2-by-2 is no longer available */
    SYMMETRIC,      /* AMD(A+A') no coletree postorder */
}

impl Strategy {
    pub fn to_int(&self) -> i32 {
        match self {
            Strategy::THRESH_SYM => 9,
            Strategy::THRESH_NNZDIAG => 12,
            Strategy::AUTO => 0,
            Strategy::UNSYMMETRIC => 1,
            Strategy::OBSOLETE => 2,
            Strategy::SYMMETRIC => 3,
        }
    }
}

pub enum Scale {
    NONE, /* no scaling */
    SUM,  /* default: divide each row by sum (abs (row))*/
    MAX,  /* divide each row by max (abs (row)) */
}

#[allow(non_camel_case_types)]
impl Scale {
    pub fn to_int(&self) -> i32 {
        match self {
            Scale::NONE => 0,
            Scale::SUM => 1,
            Scale::MAX => 2,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Ordering {
    CHOLMOD,     /* use CHOLMOD (AMD,/COLAMD then METIS)*/
    AMD,         /* use AMD,/COLAMD */
    GIVEN,       /* user-provided Qinit */
    METIS,       /* use METIS */
    BEST,        /* try many orderings pick best */
    NONE,        /* natural ordering */
    USER,        /* user-provided function */
    METIS_GUARD, /* Use METIS AMD, or COLAMD.*/
}

impl Ordering {
    pub fn to_int(&self) -> i32 {
        match self {
            Ordering::CHOLMOD => 0,
            Ordering::AMD => 1,
            Ordering::GIVEN => 2,
            Ordering::METIS => 3,
            Ordering::BEST => 4,
            Ordering::NONE => 5,
            Ordering::USER => 6,
            Ordering::METIS_GUARD => 7,
        }
    }
}

pub struct Info {
    _data: [f64; 90],
}

impl Info {
    pub fn new() -> Self {
        return Self {
            _data: array_init(|_| 0.0),
        };
    }
}

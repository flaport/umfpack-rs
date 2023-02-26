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
    pub prl: i32,                     /* print level */
    pub dense_row: f64,               /* dense row parameter */
    pub dense_col: f64,               /* dense col parameter */
    pub pivot_tolerance: f64,         /* threshold partial pivoting setting */
    pub block_size: i32,              /* BLAS-3 block size */
    pub strategy: Strategy,           /* umfpack strategy */
    pub alloc_init: f64,              /* initial allocation ratio */
    pub irstep: i32,                  /* max # of iterative refinements */
    pub compiled_with_blas: bool,     /* uses the BLAS */
    pub strategy_thresh_sym: f64,     /* symmetry threshold */
    pub ordering: Ordering,           /* ordering method to use */
    pub singletons: bool,             /* singleton filter if true */
    pub strategy_thresh_nnzdiag: f64, /* nnz(diag(A)) threshold */
    pub fixq: FixQ,                   /* fixq */
    pub amd_dense: f64,               /* for AMD ordering */
    pub sym_pivot_tolerance: f64,     /* threshold, only for diag. entries */
    pub scale: Scale,                 /* what row scaling to do */
    pub front_alloc_init: f64,        /* frontal matrix allocation ratio */
    pub droptol: i32,                 /* drop tolerance for entries in L,U */
    pub aggressive: bool,             /* whether or not to use aggressive */
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
            alloc_init: 0.7,
            irstep: 2,
            compiled_with_blas: true,
            strategy_thresh_sym: 0.3,
            ordering: Ordering::AMD,
            singletons: true,
            strategy_thresh_nnzdiag: 0.9,
            fixq: FixQ::DEFAULT,
            amd_dense: 10.0,
            sym_pivot_tolerance: 0.001,
            scale: Scale::SUM,
            front_alloc_init: 0.5,
            droptol: 0,
            aggressive: true,
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
        data[9] = self.strategy_thresh_sym as f64;
        data[10] = self.ordering.to_int() as f64;
        data[11] = self.singletons as i32 as f64;
        data[12] = self.strategy_thresh_nnzdiag as f64;
        data[13] = self.fixq.to_int() as f64;
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
pub enum FixQ {
    NOFIXQ,
    DEFAULT,
    FIXQ,
}

impl FixQ {
    pub fn to_int(&self) -> i32 {
        match self {
            FixQ::NOFIXQ => -1,
            FixQ::DEFAULT => 0,
            FixQ::FIXQ => 1,
        }
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

    /// UMFPACK_OK, or other result, returned by all routines that use Info
    pub fn umfpack_status(&self) -> f64 {
        self._data[0]
    }
    /// n_row input value, returned by all routines that use Info
    pub fn umfpack_nrow(&self) -> f64 {
        self._data[1]
    }
    /// # of entries in A, returned by all routines that use Info
    pub fn umfpack_nz(&self) -> f64 {
        self._data[2]
    }
    /// sizeof (Unit) computed in UMFPACK_*symbolic and UMFPACK_numeric:
    pub fn umfpack_size_of_unit(&self) -> f64 {
        self._data[3]
    }
    /// sizeof (int32_t), computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_int(&self) -> f64 {
        self._data[4]
    }
    /// sizeof (int64_t), computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_long(&self) -> f64 {
        self._data[5]
    }
    /// sizeof (void, computed in UMFPACK_*symbolic: *)
    pub fn umfpack_size_of_pointer(&self) -> f64 {
        self._data[6]
    }
    /// sizeof (Entry), real or complex, computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_entry(&self) -> f64 {
        self._data[7]
    }
    /// number of dense rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_ndense_row(&self) -> f64 {
        self._data[8]
    }
    /// number of empty rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_nempty_row(&self) -> f64 {
        self._data[9]
    }
    /// number of dense rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_ndense_col(&self) -> f64 {
        self._data[10]
    }
    /// number of empty rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_nempty_col(&self) -> f64 {
        self._data[11]
    }
    /// # of memory compactions, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_defrag(&self) -> f64 {
        self._data[12]
    }
    /// memory used by symbolic analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_peak_memory(&self) -> f64 {
        self._data[13]
    }
    /// size of Symbolic object, in Units, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_size(&self) -> f64 {
        self._data[14]
    }
    /// time (sec.) for symbolic analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_time(&self) -> f64 {
        self._data[15]
    }
    /// n_col input value, returned by all routines that use Info, computed in UMFPACK_*symbolic:
    pub fn umfpack_ncol(&self) -> f64 {
        self._data[16]
    }
    /// wall clock time for sym. analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_walltime(&self) -> f64 {
        self._data[17]
    }
    /// strategy used: sym, unsym, computed in UMFPACK_*symbolic:
    pub fn umfpack_strategy_used(&self) -> f64 {
        self._data[18]
    }
    /// ordering used: colamd, amd, given, computed in UMFPACK_*symbolic:
    pub fn umfpack_ordering_used(&self) -> f64 {
        self._data[19]
    }
    /// whether Q is fixed or refined, computed in UMFPACK_*symbolic:
    pub fn umfpack_qfixed(&self) -> f64 {
        self._data[31]
    }
    /// whether diagonal pivoting attempte, computed in UMFPACK_*symbolic:
    pub fn umfpack_diag_preferred(&self) -> f64 {
        self._data[32]
    }
    /// symmetry of pattern of S, computed in UMFPACK_*symbolic:
    pub fn umfpack_pattern_symmetry(&self) -> f64 {
        self._data[33]
    }
    /// nnz (S+S'), excl. diagonal, computed in UMFPACK_*symbolic:
    pub fn umfpack_nz_a_plus_at(&self) -> f64 {
        self._data[34]
    }
    /// nnz (diag (S)), computed in UMFPACK_*symbolic:
    pub fn umfpack_nzdiag(&self) -> f64 {
        self._data[35]
    }
    /// nz in L+U, if AMD ordering used, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_lunz(&self) -> f64 {
        self._data[36]
    }
    /// flops for LU, if AMD ordering used, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_flops(&self) -> f64 {
        self._data[37]
    }
    /// # of "dense" rows/cols in S+S', AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_ndense(&self) -> f64 {
        self._data[38]
    }
    /// max nz in cols of L, for AMD, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_dmax(&self) -> f64 {
        self._data[39]
    }
    /// # of column singletons, statistcs for singleton pruning
    pub fn umfpack_col_singletons(&self) -> f64 {
        self._data[56]
    }
    /// # of row singletons, statistcs for singleton pruning
    pub fn umfpack_row_singletons(&self) -> f64 {
        self._data[57]
    }
    /// size of S, statistcs for singleton pruning
    pub fn umfpack_n2(&self) -> f64 {
        self._data[58]
    }
    /// 1 if S square and symmetricly perm, statistcs for singleton pruning
    pub fn umfpack_s_symmetric(&self) -> f64 {
        self._data[59]
    }
    /// final size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_numeric_size_estimate(&self) -> f64 {
        self._data[20]
    }
    /// for symbolic & numeric, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_peak_memory_estimate(&self) -> f64 {
        self._data[21]
    }
    /// flop count, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_flops_estimate(&self) -> f64 {
        self._data[22]
    }
    /// nz in L, incl. diagonal, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_lnz_estimate(&self) -> f64 {
        self._data[23]
    }
    /// nz in U, incl. diagonal, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_unz_estimate(&self) -> f64 {
        self._data[24]
    }
    /// initial size of Numeric->Memor, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_init_estimate(&self) -> f64 {
        self._data[25]
    }
    /// peak size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_peak_estimate(&self) -> f64 {
        self._data[26]
    }
    /// final size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_final_estimate(&self) -> f64 {
        self._data[27]
    }
    /// max frontal matrix size, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_size_estimate(&self) -> f64 {
        self._data[28]
    }
    /// max # rows in any front, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_nrows_estimate(&self) -> f64 {
        self._data[29]
    }
    /// max # columns in any front, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_ncols_estimate(&self) -> f64 {
        self._data[30]
    }
    /// final size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_numeric_size(&self) -> f64 {
        self._data[40]
    }
    /// for symbolic & numeric, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_peak_memory(&self) -> f64 {
        self._data[41]
    }
    /// flop count, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_flops(&self) -> f64 {
        self._data[42]
    }
    /// nz in L, incl. diagonal, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_lnz(&self) -> f64 {
        self._data[43]
    }
    /// nz in U, incl. diagonal, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_unz(&self) -> f64 {
        self._data[44]
    }
    /// initial size of Numeric->Memor, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_init(&self) -> f64 {
        self._data[45]
    }
    /// peak size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_peak(&self) -> f64 {
        self._data[46]
    }
    /// final size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_final(&self) -> f64 {
        self._data[47]
    }
    /// max frontal matrix size, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_size(&self) -> f64 {
        self._data[48]
    }
    /// max # rows in any front, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_nrows(&self) -> f64 {
        self._data[49]
    }
    /// max # columns in any front, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_ncols(&self) -> f64 {
        self._data[50]
    }
    /// # of garbage collections, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_defrag(&self) -> f64 {
        self._data[60]
    }
    /// # of memory reallocations, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_realloc(&self) -> f64 {
        self._data[61]
    }
    /// # of costlly memory realloc's, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_costly_realloc(&self) -> f64 {
        self._data[62]
    }
    /// # of integers in LU pattern, computed in UMFPACK_numeric:
    pub fn umfpack_compressed_pattern(&self) -> f64 {
        self._data[63]
    }
    /// # of reals in LU factors, computed in UMFPACK_numeric:
    pub fn umfpack_lu_entries(&self) -> f64 {
        self._data[64]
    }
    /// numeric factorization time, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_time(&self) -> f64 {
        self._data[65]
    }
    /// nz on diagonal of U, computed in UMFPACK_numeric:
    pub fn umfpack_udiag_nz(&self) -> f64 {
        self._data[66]
    }
    /// est. reciprocal condition #, computed in UMFPACK_numeric:
    pub fn umfpack_rcond(&self) -> f64 {
        self._data[67]
    }
    /// none, max row, or sum row, computed in UMFPACK_numeric:
    pub fn umfpack_was_scaled(&self) -> f64 {
        self._data[68]
    }
    /// min (max row) or min (sum row), computed in UMFPACK_numeric:
    pub fn umfpack_rsmin(&self) -> f64 {
        self._data[69]
    }
    /// max (max row) or max (sum row), computed in UMFPACK_numeric:
    pub fn umfpack_rsmax(&self) -> f64 {
        self._data[70]
    }
    /// min abs diagonal entry of U, computed in UMFPACK_numeric:
    pub fn umfpack_umin(&self) -> f64 {
        self._data[71]
    }
    /// max abs diagonal entry of U, computed in UMFPACK_numeric:
    pub fn umfpack_umax(&self) -> f64 {
        self._data[72]
    }
    /// alloc_init parameter used, computed in UMFPACK_numeric:
    pub fn umfpack_alloc_init_used(&self) -> f64 {
        self._data[73]
    }
    /// # of forced updates, computed in UMFPACK_numeric:
    pub fn umfpack_forced_updates(&self) -> f64 {
        self._data[74]
    }
    /// numeric wall clock time, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_walltime(&self) -> f64 {
        self._data[75]
    }
    /// number of off-diagonal pivots, computed in UMFPACK_numeric:
    pub fn umfpack_noff_diag(&self) -> f64 {
        self._data[76]
    }
    /// nz in L, if no dropped entries, computed in UMFPACK_numeric:
    pub fn umfpack_all_lnz(&self) -> f64 {
        self._data[77]
    }
    /// nz in U, if no dropped entries, computed in UMFPACK_numeric:
    pub fn umfpack_all_unz(&self) -> f64 {
        self._data[78]
    }
    /// # of dropped small entries, computed in UMFPACK_numeric:
    pub fn umfpack_nzdropped(&self) -> f64 {
        self._data[79]
    }
    /// # of iterative refinement steps taken, computed in UMFPACK_solve:
    pub fn umfpack_ir_taken(&self) -> f64 {
        self._data[80]
    }
    /// # of iter. refinement steps attempted, computed in UMFPACK_solve:
    pub fn umfpack_ir_attempted(&self) -> f64 {
        self._data[81]
    }
    /// omega1, sparse backward error estimate, computed in UMFPACK_solve:
    pub fn umfpack_omega1(&self) -> f64 {
        self._data[82]
    }
    /// omega2, sparse backward error estimate, computed in UMFPACK_solve:
    pub fn umfpack_omega2(&self) -> f64 {
        self._data[83]
    }
    /// flop count for solve, computed in UMFPACK_solve:
    pub fn umfpack_solve_flops(&self) -> f64 {
        self._data[84]
    }
    /// solve time (seconds), computed in UMFPACK_solve:
    pub fn umfpack_solve_time(&self) -> f64 {
        self._data[85]
    }
    /// solve time (wall clock, seconds), computed in UMFPACK_solve:
    pub fn umfpack_solve_walltime(&self) -> f64 {
        self._data[86]
    }
    /* [51, 52, 53, 54, 55, 87, 88, 89] unused */
}

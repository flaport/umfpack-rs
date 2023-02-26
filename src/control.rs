use array_init::array_init;

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
    pub fn data(&self) -> [f64; 20] {
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

use array_init::array_init;

pub struct Info {
    pub data: [f64; 90],
}

impl Info {
    pub fn new() -> Self {
        return Self {
            data: array_init(|_| 0.0),
        };
    }

    /// UMFPACK_OK, or other result, returned by all routines that use Info
    pub fn umfpack_status(&self) -> f64 {
        self.data[0]
    }
    /// n_row input value, returned by all routines that use Info
    pub fn umfpack_nrow(&self) -> f64 {
        self.data[1]
    }
    /// # of entries in A, returned by all routines that use Info
    pub fn umfpack_nz(&self) -> f64 {
        self.data[2]
    }
    /// sizeof (Unit) computed in UMFPACK_*symbolic and UMFPACK_numeric:
    pub fn umfpack_size_of_unit(&self) -> f64 {
        self.data[3]
    }
    /// sizeof (int32_t), computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_int(&self) -> f64 {
        self.data[4]
    }
    /// sizeof (int64_t), computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_long(&self) -> f64 {
        self.data[5]
    }
    /// sizeof (void, computed in UMFPACK_*symbolic: *)
    pub fn umfpack_size_of_pointer(&self) -> f64 {
        self.data[6]
    }
    /// sizeof (Entry), real or complex, computed in UMFPACK_*symbolic:
    pub fn umfpack_size_of_entry(&self) -> f64 {
        self.data[7]
    }
    /// number of dense rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_ndense_row(&self) -> f64 {
        self.data[8]
    }
    /// number of empty rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_nempty_row(&self) -> f64 {
        self.data[9]
    }
    /// number of dense rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_ndense_col(&self) -> f64 {
        self.data[10]
    }
    /// number of empty rows, computed in UMFPACK_*symbolic:
    pub fn umfpack_nempty_col(&self) -> f64 {
        self.data[11]
    }
    /// # of memory compactions, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_defrag(&self) -> f64 {
        self.data[12]
    }
    /// memory used by symbolic analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_peak_memory(&self) -> f64 {
        self.data[13]
    }
    /// size of Symbolic object, in Units, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_size(&self) -> f64 {
        self.data[14]
    }
    /// time (sec.) for symbolic analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_time(&self) -> f64 {
        self.data[15]
    }
    /// n_col input value, returned by all routines that use Info, computed in UMFPACK_*symbolic:
    pub fn umfpack_ncol(&self) -> f64 {
        self.data[16]
    }
    /// wall clock time for sym. analysis, computed in UMFPACK_*symbolic:
    pub fn umfpack_symbolic_walltime(&self) -> f64 {
        self.data[17]
    }
    /// strategy used: sym, unsym, computed in UMFPACK_*symbolic:
    pub fn umfpack_strategy_used(&self) -> f64 {
        self.data[18]
    }
    /// ordering used: colamd, amd, given, computed in UMFPACK_*symbolic:
    pub fn umfpack_ordering_used(&self) -> f64 {
        self.data[19]
    }
    /// whether Q is fixed or refined, computed in UMFPACK_*symbolic:
    pub fn umfpack_qfixed(&self) -> f64 {
        self.data[31]
    }
    /// whether diagonal pivoting attempte, computed in UMFPACK_*symbolic:
    pub fn umfpack_diag_preferred(&self) -> f64 {
        self.data[32]
    }
    /// symmetry of pattern of S, computed in UMFPACK_*symbolic:
    pub fn umfpack_pattern_symmetry(&self) -> f64 {
        self.data[33]
    }
    /// nnz (S+S'), excl. diagonal, computed in UMFPACK_*symbolic:
    pub fn umfpack_nz_a_plus_at(&self) -> f64 {
        self.data[34]
    }
    /// nnz (diag (S)), computed in UMFPACK_*symbolic:
    pub fn umfpack_nzdiag(&self) -> f64 {
        self.data[35]
    }
    /// nz in L+U, if AMD ordering used, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_lunz(&self) -> f64 {
        self.data[36]
    }
    /// flops for LU, if AMD ordering used, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_flops(&self) -> f64 {
        self.data[37]
    }
    /// # of "dense" rows/cols in S+S', AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_ndense(&self) -> f64 {
        self.data[38]
    }
    /// max nz in cols of L, for AMD, AMD statistics, computed in UMFPACK_*symbolic:
    pub fn umfpack_symmetric_dmax(&self) -> f64 {
        self.data[39]
    }
    /// # of column singletons, statistcs for singleton pruning
    pub fn umfpack_col_singletons(&self) -> f64 {
        self.data[56]
    }
    /// # of row singletons, statistcs for singleton pruning
    pub fn umfpack_row_singletons(&self) -> f64 {
        self.data[57]
    }
    /// size of S, statistcs for singleton pruning
    pub fn umfpack_n2(&self) -> f64 {
        self.data[58]
    }
    /// 1 if S square and symmetricly perm, statistcs for singleton pruning
    pub fn umfpack_s_symmetric(&self) -> f64 {
        self.data[59]
    }
    /// final size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_numeric_size_estimate(&self) -> f64 {
        self.data[20]
    }
    /// for symbolic & numeric, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_peak_memory_estimate(&self) -> f64 {
        self.data[21]
    }
    /// flop count, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_flops_estimate(&self) -> f64 {
        self.data[22]
    }
    /// nz in L, incl. diagonal, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_lnz_estimate(&self) -> f64 {
        self.data[23]
    }
    /// nz in U, incl. diagonal, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_unz_estimate(&self) -> f64 {
        self.data[24]
    }
    /// initial size of Numeric->Memor, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_init_estimate(&self) -> f64 {
        self.data[25]
    }
    /// peak size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_peak_estimate(&self) -> f64 {
        self.data[26]
    }
    /// final size of Numeric->Memory, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_variable_final_estimate(&self) -> f64 {
        self.data[27]
    }
    /// max frontal matrix size, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_size_estimate(&self) -> f64 {
        self.data[28]
    }
    /// max # rows in any front, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_nrows_estimate(&self) -> f64 {
        self.data[29]
    }
    /// max # columns in any front, estimates computed in UMFPACK_*symbolic:
    pub fn umfpack_max_front_ncols_estimate(&self) -> f64 {
        self.data[30]
    }
    /// final size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_numeric_size(&self) -> f64 {
        self.data[40]
    }
    /// for symbolic & numeric, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_peak_memory(&self) -> f64 {
        self.data[41]
    }
    /// flop count, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_flops(&self) -> f64 {
        self.data[42]
    }
    /// nz in L, incl. diagonal, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_lnz(&self) -> f64 {
        self.data[43]
    }
    /// nz in U, incl. diagonal, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_unz(&self) -> f64 {
        self.data[44]
    }
    /// initial size of Numeric->Memor, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_init(&self) -> f64 {
        self.data[45]
    }
    /// peak size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_peak(&self) -> f64 {
        self.data[46]
    }
    /// final size of Numeric->Memory, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_variable_final(&self) -> f64 {
        self.data[47]
    }
    /// max frontal matrix size, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_size(&self) -> f64 {
        self.data[48]
    }
    /// max # rows in any front, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_nrows(&self) -> f64 {
        self.data[49]
    }
    /// max # columns in any front, exact values, (estimates shown above) computed in UMFPACK_numeric:
    pub fn umfpack_max_front_ncols(&self) -> f64 {
        self.data[50]
    }
    /// # of garbage collections, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_defrag(&self) -> f64 {
        self.data[60]
    }
    /// # of memory reallocations, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_realloc(&self) -> f64 {
        self.data[61]
    }
    /// # of costlly memory realloc's, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_costly_realloc(&self) -> f64 {
        self.data[62]
    }
    /// # of integers in LU pattern, computed in UMFPACK_numeric:
    pub fn umfpack_compressed_pattern(&self) -> f64 {
        self.data[63]
    }
    /// # of reals in LU factors, computed in UMFPACK_numeric:
    pub fn umfpack_lu_entries(&self) -> f64 {
        self.data[64]
    }
    /// numeric factorization time, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_time(&self) -> f64 {
        self.data[65]
    }
    /// nz on diagonal of U, computed in UMFPACK_numeric:
    pub fn umfpack_udiag_nz(&self) -> f64 {
        self.data[66]
    }
    /// est. reciprocal condition #, computed in UMFPACK_numeric:
    pub fn umfpack_rcond(&self) -> f64 {
        self.data[67]
    }
    /// none, max row, or sum row, computed in UMFPACK_numeric:
    pub fn umfpack_was_scaled(&self) -> f64 {
        self.data[68]
    }
    /// min (max row) or min (sum row), computed in UMFPACK_numeric:
    pub fn umfpack_rsmin(&self) -> f64 {
        self.data[69]
    }
    /// max (max row) or max (sum row), computed in UMFPACK_numeric:
    pub fn umfpack_rsmax(&self) -> f64 {
        self.data[70]
    }
    /// min abs diagonal entry of U, computed in UMFPACK_numeric:
    pub fn umfpack_umin(&self) -> f64 {
        self.data[71]
    }
    /// max abs diagonal entry of U, computed in UMFPACK_numeric:
    pub fn umfpack_umax(&self) -> f64 {
        self.data[72]
    }
    /// alloc_init parameter used, computed in UMFPACK_numeric:
    pub fn umfpack_alloc_init_used(&self) -> f64 {
        self.data[73]
    }
    /// # of forced updates, computed in UMFPACK_numeric:
    pub fn umfpack_forced_updates(&self) -> f64 {
        self.data[74]
    }
    /// numeric wall clock time, computed in UMFPACK_numeric:
    pub fn umfpack_numeric_walltime(&self) -> f64 {
        self.data[75]
    }
    /// number of off-diagonal pivots, computed in UMFPACK_numeric:
    pub fn umfpack_noff_diag(&self) -> f64 {
        self.data[76]
    }
    /// nz in L, if no dropped entries, computed in UMFPACK_numeric:
    pub fn umfpack_all_lnz(&self) -> f64 {
        self.data[77]
    }
    /// nz in U, if no dropped entries, computed in UMFPACK_numeric:
    pub fn umfpack_all_unz(&self) -> f64 {
        self.data[78]
    }
    /// # of dropped small entries, computed in UMFPACK_numeric:
    pub fn umfpack_nzdropped(&self) -> f64 {
        self.data[79]
    }
    /// # of iterative refinement steps taken, computed in UMFPACK_solve:
    pub fn umfpack_ir_taken(&self) -> f64 {
        self.data[80]
    }
    /// # of iter. refinement steps attempted, computed in UMFPACK_solve:
    pub fn umfpack_ir_attempted(&self) -> f64 {
        self.data[81]
    }
    /// omega1, sparse backward error estimate, computed in UMFPACK_solve:
    pub fn umfpack_omega1(&self) -> f64 {
        self.data[82]
    }
    /// omega2, sparse backward error estimate, computed in UMFPACK_solve:
    pub fn umfpack_omega2(&self) -> f64 {
        self.data[83]
    }
    /// flop count for solve, computed in UMFPACK_solve:
    pub fn umfpack_solve_flops(&self) -> f64 {
        self.data[84]
    }
    /// solve time (seconds), computed in UMFPACK_solve:
    pub fn umfpack_solve_time(&self) -> f64 {
        self.data[85]
    }
    /// solve time (wall clock, seconds), computed in UMFPACK_solve:
    pub fn umfpack_solve_walltime(&self) -> f64 {
        self.data[86]
    }
    /* [51, 52, 53, 54, 55, 87, 88, 89] unused */
}

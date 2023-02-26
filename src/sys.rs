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

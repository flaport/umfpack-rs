use umfpack::prelude::*;

fn main() {
    let blas_version = SuiteSparse_BLAS_library();
    println!("{blas_version}\n\n");

    println!("\n--- rust output: (option 1) ---\n");
    example4_1_rs();

    println!("\n--- rust output: (option 2) ---\n");
    example4_2_rs();

    println!("\n\n--- C output: ---\n");
    example4_c();
}

#[allow(non_snake_case)]
fn example4_1_rs() {
    let n = 5;
    let Ap = vec![0, 2, 5, 9, 10, 12];
    let Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = vec![2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0];
    let Az = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let Bx = vec![8.0, 45.0, -3.0, 3.0, 19.0];
    let Bz = vec![3.0, 3.0, 3.0, 3.0, 3.0];
    let mut Xx = vec![0.0, 0.0, 0.0, 0.0, 0.0];
    let mut Xz = vec![0.0, 0.0, 0.0, 0.0, 0.0];

    let mut symbolic = Symbolic::new();
    umfpack_zi_symbolic(n, n, &Ap, &Ai, &Ax, Some(&Az), &mut symbolic, None, None);

    let mut numeric = Numeric::new();
    umfpack_zi_numeric(
        &Ap,
        &Ai,
        &Ax,
        Some(&Az),
        &symbolic,
        &mut numeric,
        None,
        None,
    );

    umfpack_zi_solve(
        UMFPACK::A,
        &Ap,
        &Ai,
        &Ax,
        Some(&Az),
        &mut Xx,
        Some(&mut Xz),
        &Bx,
        Some(&Bz),
        &numeric,
        None,
        None,
    );

    for i in 0..(n as usize) {
        println!("X [{}] = {:.1}+{:.1}j", i, Xx[i], Xz[i]);
    }
}

#[allow(non_snake_case)]
fn example4_2_rs() {
    let n = 5;
    let Ap = vec![0, 2, 5, 9, 10, 12];
    let Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = vec![
        2.0, 1.0, 3.0, 1.0, 3.0, 1.0, -1.0, 1.0, 4.0, 1.0, 4.0, 1.0, -3.0, 1.0, 1.0, 1.0, 2.0, 1.0,
        2.0, 1.0, 6.0, 1.0, 1.0, 1.0,
    ];
    let B = vec![8.0, 3.0, 45.0, 3.0, -3.0, 3.0, 3.0, 3.0, 19.0, 3.0];
    let mut X = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let mut symbolic = Symbolic::new();
    umfpack_zi_symbolic(n, n, &Ap, &Ai, &Ax, None, &mut symbolic, None, None);

    let mut numeric = Numeric::new();
    umfpack_zi_numeric(&Ap, &Ai, &Ax, None, &symbolic, &mut numeric, None, None);

    umfpack_zi_solve(
        UMFPACK::A,
        &Ap,
        &Ai,
        &Ax,
        None,
        &mut X,
        None,
        &B,
        None,
        &numeric,
        None,
        None,
    );

    for i in (0..(2 * n as usize)).step_by(2) {
        println!("X [{}] = {:.1}+{:.1}j", i, X[i], X[i + 1]);
    }
}

#[allow(non_snake_case)]
fn example4_c() {
    unsafe {
        c::example4();
    }
}

mod c {
    extern "C" {
        pub fn example4();
    }
}

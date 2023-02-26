use umfpack::{example, solve, SuiteSparse_BLAS_library};

#[allow(non_snake_case)]
fn main() {
    let blas_version = SuiteSparse_BLAS_library();
    println!("{blas_version}");

    example();
    println!("---");

    let n = 5;
    let Ap = &[0, 2, 5, 9, 10, 12];
    let Ai = &[0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = &[2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0];
    let b = &[8.0, 45.0, -3.0, 3.0, 19.0];
    solve(n, Ap, Ai, Ax, b);
}

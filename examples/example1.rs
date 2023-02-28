use umfpack::prelude::*;

fn main() {
    let blas_version = SuiteSparse_BLAS_library();
    println!("{blas_version}\n\n");

    println!("\n--- rust output: ---\n");
    example1_rs();

    println!("\n\n--- C output: ---\n");
    example1_c();
}

#[allow(non_snake_case)]
fn example1_rs() {
    let n = 5;
    let Ap = vec![0, 2, 5, 9, 10, 12];
    let Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = vec![2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0];
    let B = vec![8.0, 45.0, -3.0, 3.0, 19.0];
    let mut X = vec![0.0, 0.0, 0.0, 0.0, 0.0];

    let mut symbolic = Symbolic::new();
    umfpack_di_symbolic(n, n, &Ap, &Ai, &Ax, &mut symbolic, None, None);

    let mut numeric = Numeric::new();
    umfpack_di_numeric(&Ap, &Ai, &Ax, &symbolic, &mut numeric, None, None);

    umfpack_di_solve(UMFPACK::A, &Ap, &Ai, &Ax, &mut X, &B, &numeric, None, None);

    for i in 0..(n as usize) {
        println!("X [{}] = {:.1}", i, X[i]);
    }
}

#[allow(non_snake_case)]
fn example1_c() {
    unsafe {
        c::example1();
    }
}

mod c {
    extern "C" {
        pub fn example1();
    }
}

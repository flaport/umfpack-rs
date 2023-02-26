use umfpack::blas::SuiteSparse_BLAS_library;
use umfpack::control::Control;
use umfpack::di::{umfpack_di_numeric, umfpack_di_solve, umfpack_di_symbolic, Numeric, Symbolic};
use umfpack::info::Info;
use umfpack::sys::UMFPACK;

#[allow(non_snake_case)]
fn main() {
    let blas_version = SuiteSparse_BLAS_library();
    println!("{blas_version}");
    let n = 5;
    let Ap = &[0, 2, 5, 9, 10, 12];
    let Ai = &[0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = &[2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0];
    let b = &[8.0, 45.0, -3.0, 3.0, 19.0];
    let x = &mut [0.0, 0.0, 0.0, 0.0, 0.0];

    let mut info = Info::new();
    let control = Control::new();
    let mut symbolic = Symbolic::new();
    umfpack_di_symbolic(n, n, Ap, Ai, Ax, &mut symbolic, &control, &mut info);

    let mut numeric = Numeric::new();
    umfpack_di_numeric(Ap, Ai, Ax, &symbolic, &mut numeric, &control, &mut info);

    umfpack_di_solve(UMFPACK::A, Ap, Ai, Ax, x, b, &numeric, &control, &mut info);

    for i in 0..(n as usize) {
        println!("x [{}] = {:.1}", i, x[i]);
    }

    println!("\n--- c-output: ---\n");
    unsafe {
        c::example();
    }
}

mod c {
    extern "C" {
        pub fn example();
    }
}

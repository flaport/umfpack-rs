use num_complex::Complex64;
use umfpack::prelude::*;

fn main() {
    let blas_version = SuiteSparse_BLAS_library();
    println!("{blas_version}\n\n");

    println!("\n--- rust output: ---\n");
    example2_rs();

    println!("\n\n--- C output: ---\n");
    example2_c();
}

#[allow(non_snake_case)]
fn example2_rs() {
    let n = 5;
    let Ap = vec![0, 2, 5, 9, 10, 12];
    let Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Az = vec![
        Complex64 { re: 2.0, im: 1.0 },
        Complex64 { re: 3.0, im: 1.0 },
        Complex64 { re: 3.0, im: 1.0 },
        Complex64 { re: -1.0, im: 1.0 },
        Complex64 { re: 4.0, im: 1.0 },
        Complex64 { re: 4.0, im: 1.0 },
        Complex64 { re: -3.0, im: 1.0 },
        Complex64 { re: 1.0, im: 1.0 },
        Complex64 { re: 2.0, im: 1.0 },
        Complex64 { re: 2.0, im: 1.0 },
        Complex64 { re: 6.0, im: 1.0 },
        Complex64 { re: 1.0, im: 1.0 },
    ];
    let Bz = vec![
        Complex64 { re: 8.0, im: 3.0 },
        Complex64 { re: 45.0, im: 3.0 },
        Complex64 { re: -3.0, im: 3.0 },
        Complex64 { re: 3.0, im: 3.0 },
        Complex64 { re: 19.0, im: 3.0 },
    ];
    let mut Xz = vec![
        Complex64 { re: 0.0, im: 0.0 },
        Complex64 { re: 0.0, im: 0.0 },
        Complex64 { re: 0.0, im: 0.0 },
        Complex64 { re: 0.0, im: 0.0 },
        Complex64 { re: 0.0, im: 0.0 },
    ];

    let mut info = Info::new();
    let control = Control::new();
    let mut symbolic = Symbolic::new();
    umfpack_zi_symbolic(
        n,
        n,
        &Ap,
        &Ai,
        &Az,
        &mut symbolic,
        Some(&control),
        Some(&mut info),
    );

    let mut numeric = Numeric::new();
    umfpack_zi_numeric(
        &Ap,
        &Ai,
        &Az,
        &symbolic,
        &mut numeric,
        Some(&control),
        Some(&mut info),
    );

    umfpack_zi_solve(
        UMFPACK::A,
        &Ap,
        &Ai,
        &Az,
        &mut Xz,
        &Bz,
        &numeric,
        Some(&control),
        Some(&mut info),
    );

    for i in 0..(n as usize) {
        let p = if Xz[i].im < 0.0 { "" } else { "+" };
        println!("x [{}] = {:.1}{}{:.1}j", i, Xz[i].re, p, Xz[i].im);
    }

    println!("Solve time: {}", info.umfpack_solve_walltime());
}

#[allow(non_snake_case)]
fn example2_c() {
    unsafe {
        c::example2();
    }
}

mod c {
    extern "C" {
        pub fn example2();
    }
}

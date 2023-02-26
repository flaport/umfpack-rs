use umfpack::prelude::*;

#[allow(non_snake_case)]
fn main(){
    let n = 5;
    let Ap = &[0, 2, 5, 9, 10, 12];
    let Ai = &[0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let Ax = &[2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0];
    let Az = &[0.0, 0.0, 0.0,  0.0, 0.0, 0.0,  0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let Bx = &[8.0, 45.0, -3.0, 3.0, 19.0];
    let Bz = &[3.0, 1.0, -5.0, 2.0, 23.0];
    let Xx = &mut [0.0, 0.0, 0.0, 0.0, 0.0];
    let Xz = &mut [0.0, 0.0, 0.0, 0.0, 0.0];

    let mut info = Info::new();
    let control = Control::new();
    let mut symbolic = Symbolic::new();
    umfpack_zi_symbolic(n, n, Ap, Ai, Ax, Az, &mut symbolic, &control, &mut info);

    let mut numeric = Numeric::new();
    umfpack_zi_numeric(Ap, Ai, Ax, Az, &symbolic, &mut numeric, &control, &mut info);

    umfpack_zi_solve(UMFPACK::A, Ap, Ai, Ax, Az, Xx, Xz, Bx, Bz, &numeric, &control, &mut info);

    for i in 0..(n as usize) {
        println!("x [{}] = {:.1}+{:.1}j", i, Xx[i], Xz[i]);
    }

    println!("Solve time: {}", info.umfpack_solve_walltime());
}

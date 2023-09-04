#[allow(non_snake_case)]
fn main() {
    use umfpack::prelude::*;

    let n = 5;
    let Ap = vec![0, 2, 5, 9, 10, 12]; // column pointers of CSC sparse nxn matrix
    let Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4]; // row indices of CSC sparse matrix
    let Ax = vec![2.0, 3.0, 3.0, -1.0, 4.0, 4.0, -3.0, 1.0, 2.0, 2.0, 6.0, 1.0]; // values of CSC sparse matrix
    let b = vec![8.0, 45.0, -3.0, 3.0, 19.0]; // dense target
    let mut x = vec![0.0, 0.0, 0.0, 0.0, 0.0]; // initial value for unknown x
    let control = Control::new(); // default control parameters
    let mut info = Info::new(); // empty info buffer

    // solving the system Ax=b happens in three steps:

    // 1. Symbolic Analyzation of the sparse system
    let mut symbolic = Symbolic::new();
    umfpack_di_symbolic(
        n, // m
        n, // n
        &Ap,
        &Ai,
        &Ax,
        &mut symbolic,
        Some(&control),
        Some(&mut info),
    );

    // 2. Numeric Analyzation of the sparse system
    let mut numeric = Numeric::new();
    umfpack_di_numeric(
        &Ap,
        &Ai,
        &Ax,
        &symbolic,
        &mut numeric,
        Some(&control),
        Some(&mut info),
    );

    // 3. Solving of the sparse system
    umfpack_di_solve(
        UMFPACK::A, // solve the system Ax=b
        &Ap,
        &Ai,
        &Ax,
        &mut x,
        &b,
        &numeric,
        Some(&control),
        Some(&mut info),
    );

    println!("symbolic walltime: {}", info.umfpack_symbolic_walltime());
    println!("numeric walltime: {}", info.umfpack_numeric_walltime());
    println!("solve walltime: {}", info.umfpack_solve_walltime());

    for i in 0..(n as usize) {
        println!("x [{}] = {:.1}", i, x[i]);
    }
}

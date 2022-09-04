mod data;
mod ops;
use data::{Mat, R, Vector};
use ops::{prod, qprod};


fn equals(u: &Vector, v: &Vector) -> bool {
    (u.len() == v.len()) && 
        u.iter().zip(v.iter()).all(|(&x,&y)| x==y)
}

fn cgb_solve(A: Mat, B: Mat, n: usize, m: usize, k: usize) -> Option<(Vector, Vector)> {
    let mut x: Vector = Vec::new();
    let mut y: Vector = Vec::new();
    x.resize(n, R::Int(0));
    y.resize(k, R::Int(0));

    let qA = A.conjugate();
    let qB = B.conjugate();

    let mut Ax = x.clone();
    let mut By = y.clone();

    for iteration in 0..1000 {
        // y = B' ⊗' (A ⊗ x)
        prod(&A, &x, &mut Ax);
        qprod(&qB, &Ax, &mut y);

        // x = A' ⊗' (B ⊗ y)
        prod(&B, &y, &mut By);
        qprod(&qA, &By, &mut x);

        // check A⊗x == B⊗y
        if equals(&Ax, &By) {
            return Some((x,y));
        }
    }
    return None;
}
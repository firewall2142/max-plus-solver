#![allow(non_snake_case)]
pub mod data;
mod ops;
pub mod parser;
use data::{Mat, R, Vector};
use ops::{prod, qprod};
use std::iter;

/// The variables need to describe the problem A⊗x == B⊗y
/// for the CGB algorithm
pub struct CgbProblem<'a> {
    A: &'a Mat,
    B: &'a Mat,
    x_init: &'a Option<Vector>
}

/// The variables need to describe the problem A⊗x == B⊗x
/// for the BZ algorithm
#[derive(Debug)]
pub struct BzProblem { 
    // didn't use reference because BZ algo acts upon the input values
    // use clone if you don't want input values to change
    A: Mat,
    B: Mat,
    x_init: Option<Vector>
}

pub fn cgb_solve(problem : CgbProblem) -> Option<(Vector, Vector)> {
    let (A, B) = (problem.A, problem.B);

    let mut x: Vector = problem.x_init.clone()
            .unwrap_or_else(|| iter::repeat(R::Int(0)).take(A.ncols).collect());
    let mut y: Vector = iter::repeat(R::Int(0)).take(B.ncols).collect();

    let qA = &A.conjugate();
    let qB = &B.conjugate();

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
        if Ax == By {
            eprintln!("Took {} iterations", iteration);
            return Some((x,y));
        }

        eprintln!("iteration = {}", iteration);
    }
    return None;
}

// find x s.t. A⊗x == B⊗x
pub fn solve(problem : BzProblem) -> Option<Vector> {
    let A = problem.A;
    let B = problem.B;
    let x_init = problem.x_init;
    debug_assert_eq!(A.emp_val, R::Ninf);
    debug_assert_eq!((A.nrows, A.ncols, A.emp_val), (B.nrows, B.ncols, B.emp_val));
    
    let m = A.nrows;
    let n = A.ncols;

    let mut Cstore = A.store;
    Cstore.extend(B.store.into_iter());
    let C = Mat {
        store : Cstore,
        nrows : 2*m,
        ncols : n,
        emp_val : R::Ninf,
    };
    
    let Dstore = 
        (0..(2*m))
            .map(|x| vec![(x%m, R::Int(0))])
            .collect();
    let D = Mat {
        store: Dstore,
        nrows: 2*m,
        ncols: m,
        emp_val: R::Ninf,
    };

    let cgb_problem = CgbProblem {A: &C, B: &D, x_init:&x_init};
    cgb_solve(cgb_problem).map(|x| x.0)

}

#[cfg(test)]
mod test{

    use crate::{data::{Mat, R, Vector}, solve, CgbProblem};
    use crate::ops;

    #[test]
    fn check_cgb_solve() {
        let A = Mat::new(&vec![
            vec![R::Int(3), R::Ninf  , R::Int(0)],
            vec![R::Int(1), R::Int(1), R::Int(0)],
            vec![R::Ninf  , R::Int(1), R::Int(2)],
        ]);

        println!("A={:?}", A);
    
        let B = Mat::new(&vec![
            vec![R::Int(1), R::Int(1)],
            vec![R::Int(3), R::Int(2)],
            vec![R::Int(3), R::Int(1)],
        ]);
    
        let x_init : Vector = vec![R::Int(5), R::Int(3), R::Int(1)];
        let cgb_problem = CgbProblem {A:&A, B:&B, x_init:&Some(x_init)};
        let (x,y) = crate::cgb_solve(cgb_problem).unwrap();
        println!("x={:?} y={:?}", x, y);
    }

    #[test]
    fn check_solve() {
        use R::*;
        let A = Mat::new(&vec![
            vec![Int(0), Int(1), Ninf,   Ninf],
            vec![Ninf,   Int(0), Int(1), Ninf],
        ]);
        let B = Mat::new(&vec![
            vec![Ninf,   Ninf,   Int(0), Int(2)],
            vec![Ninf,   Ninf,   Ninf,   Int(0)],
        ]);
        
        let x_init = None;
        
        let bzproblem = crate::BzProblem { A: A.clone(), B: B.clone(), x_init: x_init.clone() };
        let x = match solve(bzproblem) {
            None => panic!("Didn't expect result to be empty!!"),
            Some(x) => x
        };

        println!("x = {:?}", x);

        let mut Ax = Vec::new();
        let mut Bx = Vec::new();
        ops::prod(&A, &x, &mut Ax);
        ops::prod(&B, &x, &mut Bx);
        debug_assert_eq!(Ax, Bx);
        
    }
}

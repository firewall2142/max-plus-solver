use std::cmp::{max, min};
use super::data::{Mat, R, Vector};

// TODO: use a generic for matrix multiplication

// x⊗y = x + y {Ninf + Pinf = Ninf}
fn otimes(x: R, y: R) -> R {
    match (x, y) {
        (R::Int(u), R::Int(v)) => R::Int(u+v),
        (R::Ninf, _) | (_, R::Ninf) => R::Ninf, // this must occure
        (_, R::Pinf) | (R::Pinf, _) => R::Pinf  // above this
    }
}
// x⊕y = max(x,y)
fn oplus(x: R, y: R) -> R {
    match (x, y) {
        (R::Int(a), R::Int(b)) => R::Int(max(a,b)),
        (R::Ninf, z) | (z, R::Ninf) => z,
        (R::Pinf, _) | (_, R::Pinf) => R::Pinf
    }
}

// x⊗'y = x + y {Ninf + Pinf = Pinf}
fn qotimes(x: R, y: R) -> R {
    match(x, y) {
        (R::Int(u), R::Int(v)) => R::Int(u+v),
        (_, R::Pinf) | (R::Pinf, _) => R::Pinf, // this must occur
        (R::Ninf, _) | (_, R::Ninf) => R::Ninf  // above this
    }
}
// x⊕'y = min(x,y)
fn qoplus(x: R, y: R) -> R {
    match (x, y) {
        (R::Int(a), R::Int(b)) => R::Int(min(a,b)),
        (R::Pinf, z) | (z, R::Pinf) => z,
        (R::Ninf, _) | (_, R::Ninf) => R::Ninf
    }
}

// out <- A⊗x
pub fn prod(A: &Mat, x: &Vector, out: &mut Vector) {
    out.resize(A.nrows, R::Ninf);
    for i in 0..A.nrows {
        out[i] = 
            A.store[i].iter()
                .map(|(index,value)| otimes(value.clone(), x[index.clone()]))
                .fold(R::Ninf, |acc, item| oplus(acc, item))
    }
}

// out <- A⊗'x
pub fn qprod(A: &Mat, x: &Vector, out: &mut Vector) {
    out.resize(A.nrows, R::Pinf);
    for i in 0..A.nrows {
        out[i] = 
            A.store[i].iter()
                .map(|(index,value)| qotimes(value.clone(), x[index.clone()]))
                .fold(R::Pinf, |acc, item| qoplus(acc, item))
    }
}

mod test {

    #[test]
    fn prod_qprod_test() {
        use super::*;
        use R::*;

        let A = Mat::new(&vec![
            vec![R::Int(3), R::Ninf  , R::Int(0)],
            vec![R::Int(1), R::Int(1), R::Int(0)],
            vec![R::Ninf  , R::Int(1), R::Int(2)],
        ]);
        let x : Vector = vec![R::Int(5), R::Int(3), R::Int(1)];
        
        let mut Ax : Vector = Vec::new();
        prod(&A, &x, &mut Ax);
        
        println!("Ax = {:?}", Ax);
        debug_assert_eq!(Ax, vec![Int(8), Int(6), Int(4)]);

        let B = Mat::new(&vec![
            vec![R::Int(1), R::Int(1)],
            vec![R::Int(3), R::Int(2)],
            vec![R::Int(3), R::Int(1)],
        ]);

        let mut y : Vector = Vec::new();
        qprod(&B.conjugate(), &Ax, &mut y);
        println!("y = B' ⊗' (A ⊗ x) = {:?}", Ax);
        debug_assert_eq!(y, vec![Int(1), Int(3)]);

    }
}
use std::cmp::{max, min};
use data::{Matrix, R, Vector};

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
fn prod(A : &Matrix, x : &Vector, out: &mut Vector) {
    for i in 0..x.len() {
        out[i] = 
            A[i].iter()
                .map(|iv| otimes(iv.value, x[iv.index]))
                .fold(R::Ninf, |acc, item| oplus(acc, item))
    }
}

fn qprod(A : &Matrix, x : &Vector, out: &mut Vector) {
    for i in 0..x.len() {
        out[i] = 
            A[i].iter()
                .map(|iv| qotimes(iv.value, x[iv.index]))
                .fold(R::Pinf, |acc, item| qoplus(acc, item))
    }
}

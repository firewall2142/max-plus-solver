use std::cmp::{max};

#[derive(Copy, Clone)]
enum R { Pinf, Int(i32), Ninf}

// #[derive(Copy, Clone)]
struct IndVal {
    index : usize,
    value : R
}

type Matrix = Vec<Vec<IndVal>>;
type Vector = Vec<R>;

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

// out <- A⊗x
fn prod(A : &Matrix, x : &Vector, out: &mut Vector) {
    for i in 0..x.len() {
        let t = A[i].iter().map(|iv| otimes(iv.value, x[iv.index]));
    }
}

fn main() {

}

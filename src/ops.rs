use std::cmp::{max, min};
use super::data::{Mat, R, Vector};

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
fn prod(A : &Mat, x : &Vector, out: &mut Vector) {
    let A = A.getRowMajor();
    let data = match A.store {
        crate::data::MatStore::RowMajor(store) => store,
        crate::data::MatStore::Sparse(_) => panic!("sparse unexpected!"),
    };
    for i in 0..x.len() {
        out[i] = 
            data[i].iter()
                .map(|(index,value)| otimes(value.clone(), x[index.clone()]))
                .fold(R::Ninf, |acc, item| oplus(acc, item))
    }
}

fn qprod(A : &Mat, x : &Vector, out: &mut Vector) {
    let A = A.getRowMajor();
    let data = match A.store {
        crate::data::MatStore::RowMajor(store) => store,
        crate::data::MatStore::Sparse(_) => panic!("sparse unexpected!"),
    };
    for i in 0..x.len() {
        out[i] = 
           data[i].iter()
                .map(|(index,value)| qotimes(value.clone(), x[index.clone()]))
                .fold(R::Pinf, |acc, item| qoplus(acc, item))
    }
}

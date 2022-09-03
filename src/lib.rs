mod data;
mod ops;
use data::{Mat, R, Vector};

// return the greatest solution (x) of A⊗x <= B⊗y
fn principal_solution(A : Mat, B : Mat, y : Mat) {

}

fn cgb_solve(A: Mat, B: Mat, n: usize, m: usize, k: usize) {
    let mut x: Vector = Vec::new();
    let mut y: Vector = Vec::new();
    x.resize(n, R::Int(0));
    y.resize(k, R::Int(0));

    for iteration in 0..1000 {
        
    }
}
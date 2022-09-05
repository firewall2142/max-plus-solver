use std::ops::Neg;
use std::iter;

#[derive(Copy, Clone, Debug)]
pub enum R { Pinf, Int(i32), Ninf}

impl Neg for R {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            R::Pinf => R::Ninf,
            R::Int(x) => R::Int(-x),
            R::Ninf => R::Pinf,
        }
    }
}
impl PartialEq for R {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (R::Pinf, R::Pinf) | (R::Ninf, R::Ninf) => true,
            (R::Int(x), R::Int(y)) => x==y,
            _ => false
        }
    }
}


pub type Vector = Vec<R>;
pub type MatStore = Vec<Vec<(usize,R)>>;

#[derive(Debug, Clone)]
pub struct Mat {
    pub store : MatStore,
    pub nrows : usize,
    pub ncols : usize,
    // pub nvals : usize,   // number of entries
    pub emp_val : R,   // empty values
}

impl Mat {
    pub fn new(v : &Vec<Vec<R>>) -> Mat {
        let nrows = v.len();
        let ncols = v[0].len();
        let emp_val = R::Ninf;
        let mut store: MatStore = iter::repeat_with(Vec::new).take(nrows).collect();
        for (row, colvec) in v.iter().enumerate() {
            for (col, val) in colvec.iter().enumerate() {
                match val {
                    R::Ninf => (),
                    _ => store[row].push((col, val.clone()))
                }
            }
        }

        return Mat { store, nrows, ncols, emp_val };
    }

    pub fn conjugate (&self) -> Mat {
        // XXX: could be optimized by reservering capacity beforehand
        let sparse_rep = // : Vec<_> = 
            self.store
                .iter().enumerate()
                .flat_map(|(row, colval)| {
                    colval.iter().map(
                        move |(col, val)| { (col.clone(), row.clone(), -(val.clone())) }
                    )
                });
                //.collect();
        // sparse_rep.sort_unstable_by_key(|(r,c,_)| (r,c));
        
        let nrows = self.ncols;
        let ncols = self.nrows;
        let emp_val = -self.emp_val;
        let mut store : MatStore  = iter::repeat_with(Vec::new).take(nrows).collect();
        for (r,c,v) in sparse_rep {
            store[r].push((c,v));
        }
        return Mat { store, nrows, ncols, emp_val};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let A = Mat::new(&vec![
            vec![R::Int(3), R::Ninf  , R::Int(0)],
            vec![R::Int(1), R::Int(1), R::Int(0)],
            vec![R::Ninf  , R::Int(1), R::Int(2)],
        ]);
        use R::*;
        let expected = Mat { 
            store: vec![
                vec![(0, Int(3)), (2, Int(0))], 
                vec![(0, Int(1)), (1, Int(1)), (2, Int(0))], 
                vec![(1, Int(1)), (2, Int(2))]],
            nrows: 3, 
            ncols: 3, 
            emp_val: Ninf };
        assert_eq!(A.store, expected.store);
        assert_eq!(A.nrows, expected.nrows);
        assert_eq!(A.ncols, expected.ncols);
        assert_eq!(A.emp_val, expected.emp_val);
    }
}
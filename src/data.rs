use std::ops::Neg;

#[derive(Copy, Clone)]
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


pub type Vector = Vec<R>;

pub enum MatStore  {
    Sparse(Vec<(usize,usize,R)>),   // row, column, valuee
    RowMajor(Vec<Vec<(usize,R)>>),  // column, value
}

pub struct Mat {
    pub store : MatStore,
    pub nrows : usize,
    pub ncols : usize,
    pub emp_val : R   // empty values
}

impl Mat {
    pub fn conjugate (self) -> Mat {
        let store = match self.store {
            MatStore::Sparse(mut sparse) => {
                for i in 0..sparse.len() {
                    let (r,c,v) = sparse[i];
                    sparse[i] = (c,r,-v);
                }
                MatStore::Sparse(sparse)
            }
            MatStore::RowMajor(_) => return self.getSparse().conjugate(),
        };
        let emp_val = -self.emp_val;
        let (nrows, ncols) = (self.ncols, self.nrows);
        return Mat {store, emp_val, nrows, ncols};
    }
    fn getSparse (self) -> Mat {
        let store = 
            match self.store {
                MatStore::Sparse(_) => self.store,
                MatStore::RowMajor(rmat) => {
                    let mut result = Vec::new();
                    for i in 0..rmat.len() {
                        for colval in rmat[i].iter() {
                            result.push((i, colval.0, colval.1));
                        }
                    }
                    MatStore::Sparse(result)
                }
            };
        return Mat { store, ..self };
    }
    pub fn getRowMajor (self) -> Mat {
        let store = match self.store {
            MatStore::RowMajor(_) => return self,
            MatStore::Sparse(sparse) => {
                use std::iter::{repeat_with};
                let mut res: Vec<Vec<(usize, R)>> = repeat_with(Vec::new).take(self.nrows).collect();
                for (r, c, v) in sparse.into_iter() {
                    res[r].push((c,v));
                }
                MatStore::RowMajor(res)
            },
        };
        return Mat { store, ..self};
    }
}

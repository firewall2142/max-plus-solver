#[derive(Copy, Clone)]
pub enum R { Pinf, Int(i32), Ninf}

// #[derive(Copy, Clone)]
pub struct IndVal {
    pub index : usize,
    pub value : R
}

pub type Vector = Vec<R>;

pub enum MatStore <T> {
    Sparse(Vec<(usize,usize,T)>),   // row, column, valuee
    RowMajor(Vec<Vec<(usize,T)>>),  // column, value
    ColMajor(Vec<Vec<(usize,T)>>),  // row, value
}

pub struct Mat<T> {
    store : MatStore<T>,
    nrows : usize,
    ncols : usize,
    emp_val : T   // empty values
}

fn convertToSparse<T> (matrix: Mat<T>) -> Mat<T> {
    let store = 
        match matrix.store {
            MatStore::Sparse(_) => matrix.store,
            MatStore::RowMajor(rmat) => {
                let result = Vec::new();
                for i in 0..rmat.len() {
                    for colval in rmat[i].iter() {
                        result.push((i, colval.0, colval.1));
                    }
                }
                MatStore::Sparse(result)
            },
            MatStore::ColMajor(cmat) => {
                let result = Vec::new();
                for j in 0..cmat.len() {
                    for rowval in cmat[j].iter() {
                        result.push((rowval.0, j, rowval.1));
                    }                
                }
                MatStore::Sparse(result)
            }
        };
    return Mat { store, ..matrix };
}
fn convertToRowMajor<T> (matrix: Mat<T>) -> Mat<T> {
    let store = match matrix.store {
        MatStore::RowMajor(_) => return matrix,
        MatStore::ColMajor(_) => return convertToRowMajor(convertToSparse(matrix)),
        MatStore::Sparse(sparse) => {
            use std::iter::{repeat_with};
            let res: Vec<Vec<(usize,T)>> = repeat_with(Vec::new).take(matrix.nrows).collect();
            for (r, c, v) in sparse.into_iter() {
                res[r].push((c,v));
            }
            MatStore::RowMajor(res)
        },
    };
    return Mat { store, ..matrix};
}
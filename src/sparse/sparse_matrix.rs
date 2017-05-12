use std::collections::HashMap;
use std::fmt;

use num::{Num, Zero};

use matrix::{Matrix, ReadOrder};

#[derive(Clone, Debug)]
pub struct SparseMatrix<T> {
    pub read_order: ReadOrder,
    m: usize,
    n: usize,
    pub mat: HashMap<(usize, usize), T>,
}

impl<T: Clone + Num + Zero> SparseMatrix<T> {
    /// Create a new sparse matrix from a hashmap of index tuples to `Num`.
    #[inline]
    pub fn new(mat: HashMap<(usize, usize), T>, m: usize, n: usize)
        -> SparseMatrix<T>
    {
        SparseMatrix {
            read_order: ReadOrder::RowMajor,
            m, n, mat,
        }
    }

    /// Flip the read order. Toggles between row major and column major.
    #[inline]
    pub fn flip_read_order(mut self) -> Self {
        self.read_order = match self.read_order {
            ReadOrder::RowMajor => ReadOrder::ColMajor,
            ReadOrder::ColMajor => ReadOrder::RowMajor,
        };
        self
    }
}

impl<T: Clone + Num + Zero> Matrix<T> for SparseMatrix<T> {
    fn is_symmetric(&self) -> bool {
        // TODO stub
        false
    }

    fn is_orthogonal(&self) -> bool {
        // TODO stub
        false
    }

    fn is_diagonal(&self) -> bool {
        for &(i, j) in self.mat.keys() {
            if i != j { return false }
        }
        true
    }

    fn is_lower_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_unilower_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_strictly_lower_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_lower_hessenberg(&self) -> bool {
        // TODO stub
        false
    }

    fn is_upper_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_uniupper_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_strictly_upper_triangular(&self) -> bool {
        // TODO stub
        false
    }

    fn is_upper_hessenberg(&self) -> bool {
        // TODO stub
        false
    }

    fn trace(&self) -> T {
        let mut trace = T::zero();
        let mut i = 0;
        loop {
            match self.element(i, i) {
                Some(e) => {
                    trace = trace + e;
                    i += 1;
                },
                None => return trace,
            }
        }
    }

    fn transpose(self) -> Self {
        self.flip_read_order()
    }

    fn rows(&self) -> usize {
        match self.read_order {
            ReadOrder::RowMajor => self.m,
            ReadOrder::ColMajor => self.n,
        }
    }

    fn cols(&self) -> usize {
        match self.read_order {
            ReadOrder::RowMajor => self.n,
            ReadOrder::ColMajor => self.m,
        }
    }

    fn element(&self, i: usize, j: usize) -> Option<T> {
        if i > self.m && j > self.n { return None }
        match self.mat.get(&(i, j)) {
            Some(v) => Some(v.clone()),
            None => Some(Zero::zero()),
        }
    }

    fn elements(&self) -> Vec<T> {
        let mut elements: Vec<T> = Vec::with_capacity(self.m*self.n);
        for i in 0..self.m {
            for j in 0..self.n {
                elements.push(self.element(i, j)
                              .expect("SparseMatrix::element"));
            }
        }
        elements
    }
}

impl<T: Clone + Num + Zero> IntoIterator for SparseMatrix<T> {
    type Item = T;
    type IntoIter = SparseMatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SparseMatrixIntoIterator { mat: self, i: 0, j: 0 }
    }
}

impl<T> fmt::Display for SparseMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SparseMatrix(m: {}, n: {})", self.m, self.n)
    }
}

pub struct SparseMatrixIntoIterator<T> {
    mat: SparseMatrix<T>,
    i: usize,
    j: usize,
}

impl<T: Clone + Num + Zero> Iterator for SparseMatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;
        if self.j < self.mat.n {
            if self.i < self.mat.m {
                match self.mat.read_order {
                    ReadOrder::RowMajor => {
                        result = self.mat.element(self.i, self.j);
                    },
                    ReadOrder::ColMajor => {
                        result = self.mat.element(self.j, self.i);
                    },
                }
                self.i += 1;
            } else {
                self.i = 0;
                self.j += 1;
            }
        }
        result
    }
}

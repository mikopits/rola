use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use ::{Num, Zero};
use ::{Matrix, ReadOrder};

#[derive(Clone, Debug)]
pub struct SparseMatrix<T> where T: Copy {
    pub read_order: ReadOrder,
    m: usize,
    n: usize,
    pub mat: Rc<RefCell<HashMap<(usize, usize), Cell<T>>>>,
}

impl<T: Clone + Copy + Num + Zero> SparseMatrix<T> {
    /// Create a new empty sparse matrix.
    #[inline]
    pub fn new(m: usize, n: usize) -> SparseMatrix<T> {
        SparseMatrix {
            read_order: ReadOrder::RowMajor,
            m, n,
            mat: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Create a new sparse matrix from a `Vec` of tuples containing the
    /// the indeces of the element and the number in the order (i, j, a_ij).
    #[inline]
    pub fn from_tuple(mat: Vec<(usize, usize, T)>, m: usize, n: usize)
        -> SparseMatrix<T>
    {
        let mut map: HashMap<(usize, usize), Cell<T>> = HashMap::new();
        for (i, j, a_ij) in mat {
            map.insert((i, j), Cell::new(a_ij));
        }
        SparseMatrix {
            read_order: ReadOrder::RowMajor,
            m, n,
            mat: Rc::new(RefCell::new(map)),
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

impl<T: Clone + Copy + Num> Matrix<T> for SparseMatrix<T> {
    fn is_symmetric(&self) -> bool {
        // TODO stub
        false
    }

    fn is_orthogonal(&self) -> bool {
        // TODO stub
        false
    }

    fn is_diagonal(&self) -> bool {
        for &(i, j) in self.mat.borrow().keys() {
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
            match self.get(i, i) {
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

    fn get(&self, i: usize, j: usize) -> Option<T> where T: Zero {
        if i >= self.rows() || j >= self.cols() { return None }
        match self.read_order {
            ReadOrder::RowMajor => {
                match self.mat.borrow().get(&(i, j)) {
                    Some(v) => Some(v.get()),
                    None => Some(Zero::zero()),
                }
            },
            ReadOrder::ColMajor => {
                match self.mat.borrow().get(&(j, i)) {
                    Some(v) => Some(v.get()),
                    None => Some(Zero::zero()),
                }
            },
        }
    }

    fn set(&self, i: usize, j: usize, val: T) -> Option<T> {
        if i >= self.rows() || j >= self.cols() { return None }
        match self.read_order {
            ReadOrder::RowMajor => {
                self.mat.borrow_mut().insert((i, j), Cell::new(val));
            },
            ReadOrder::ColMajor => {
                self.mat.borrow_mut().insert((j, i), Cell::new(val));
            },
        }
        Some(val)
    }

    fn elements(&self) -> Vec<T> {
        let mut elements: Vec<T> = Vec::with_capacity(self.m*self.n);
        for i in 0..self.m {
            for j in 0..self.n {
                elements.push(self.get(i, j).unwrap());
            }
        }
        elements
    }
}

impl<T: Clone + Copy + Num + Zero> IntoIterator for SparseMatrix<T> {
    type Item = T;
    type IntoIter = SparseMatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SparseMatrixIntoIterator { mat: self, i: 0, j: 0 }
    }
}

impl<T: Copy + fmt::Debug> fmt::Display for SparseMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SparseMatrix: {:?}", self)
    }
}

pub struct SparseMatrixIntoIterator<T> where T: Copy {
    mat: SparseMatrix<T>,
    i: usize,
    j: usize,
}

impl<T: Clone + Copy + Num + Zero> Iterator for SparseMatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;
        if self.j < self.mat.n {
            if self.i < self.mat.m {
                result = self.mat.get(self.i, self.j);
                self.i += 1;
            } else {
                self.i = 0;
                self.j += 1;
            }
        }
        result
    }
}

use std::fmt;
use std::marker::PhantomData;
use num::{Num, Zero};
use matrix::Matrix;

#[derive(Clone, Debug)]
pub struct ZeroMatrix<T> {
    m: usize,
    n: usize,
    mat: PhantomData<T>,
}

impl<T: Clone + Num + Zero> ZeroMatrix<T> {
    pub fn new(m: usize, n: usize) -> ZeroMatrix<T> {
        ZeroMatrix { m, n, mat: PhantomData }
    }
}

impl<T: Clone + Num + Zero> Matrix<T> for ZeroMatrix<T> {
    fn is_symmetric(&self) -> bool { self.is_square() }

    fn is_orthogonal(&self) -> bool { false }

    fn is_diagonal(&self) -> bool { true }

    fn is_lower_triangular(&self) -> bool { self.is_square() }

    fn is_unilower_triangular(&self) -> bool { false }

    fn is_strictly_lower_triangular(&self) -> bool { self.is_square() }

    fn is_lower_hessenberg(&self) -> bool { self.is_square() }

    fn is_upper_triangular(&self) -> bool { self.is_square() }

    fn is_uniupper_triangular(&self) -> bool { false }

    fn is_strictly_upper_triangular(&self) -> bool { self.is_square() }

    fn is_upper_hessenberg(&self) -> bool { self.is_square() }

    fn trace(&self) -> T { Zero::zero() }

    fn transpose(self) -> Self {
        Self::new(self.n, self.m)
    }

    fn rows(&self) -> usize { self.m }

    fn cols(&self) -> usize { self.n }

    fn element(&self, i: usize, j: usize) -> Option<T> {
        if (0..self.m).any(|x| x == i)
        && (0..self.n).any(|y| y == j) {
            return Some(Zero::zero())
        }
        None
    }

    fn elements(&self) -> Vec<T> {
        vec![T::zero(); self.m*self.n]
    }
}

impl<T: Clone + Num + Zero> IntoIterator for ZeroMatrix<T> {
    type Item = T;
    type IntoIter = ZeroMatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ZeroMatrixIntoIterator { index: 0, mat: self }
    }

}

impl<T> fmt::Display for ZeroMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZeroMatrix(m: {}, n: {})", self.m, self.n)
    }
}

pub struct ZeroMatrixIntoIterator<T> {
    index: usize,
    mat: ZeroMatrix<T>,
}

impl<T: Clone + Num + Zero> Iterator for ZeroMatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;
        if self.index < self.mat.rows() * self.mat.cols() {
            result = Some(Zero::zero())
        }
        self.index += 1;
        result
    }
}

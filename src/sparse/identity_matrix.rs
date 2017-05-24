use std::fmt;
use std::marker::PhantomData;
use ::{FromPrimitive, Num, One, Zero};
use ::Matrix;

#[derive(Clone, Debug)]
pub struct IdentityMatrix<T> {
    n: usize,
    mat: PhantomData<T>,
}

impl<T: Num + One + Zero> IdentityMatrix<T> {
    pub fn new(n: usize) -> IdentityMatrix<T> {
        IdentityMatrix { m: n, mat: PhantomData }
    }
}

impl<T: Clone + Num + One + Zero + FromPrimitive> Matrix<T> for IdentityMatrix<T> {
    fn is_symmetric(&self) -> bool { true }

    fn is_orthogonal(&self) -> bool { true }

    fn is_diagonal(&self) -> bool { true }

    fn is_lower_triangular(&self) -> bool { true }

    fn is_unilower_triangular(&self) -> bool { true }

    fn is_strictly_lower_triangular(&self) -> bool { false }

    fn is_lower_hessenberg(&self) -> bool { true }

    fn is_upper_triangular(&self) -> bool { true }

    fn is_uniupper_triangular(&self) -> bool { true }

    fn is_strictly_upper_triangular(&self) -> bool { false }

    fn is_upper_hessenberg(&self) -> bool { true }

    fn trace(&self) -> T {
        T::from_usize(self.n).expect("IdentityMatrix::trace")
    }

    fn transpose(self) -> Self { self }

    fn rows(&self) -> usize { self.n }

    fn cols(&self) -> usize { self.n }

    fn get(&self, i: usize, j: usize) -> Option<T> {
        if i >= self.rows() || j >= self.cols() { return None }
        if i != j {
            Some(Zero::zero())
        } else {
            Some(One::one())
        }
    }

    fn set(&self, i: usize, j: usize, _val: T) -> Option<T> {
        if i >= self.rows() || j >= self.cols() { return None }
        // FIXME: Turn IdentityMatrix into a SparseMatrix.
        panic!("Cannot set a value in an IdentityMatrix")
    }

    fn elements(&self) -> Vec<T> {
        let mut v = vec![T::zero(); self.n*self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                if i == j {
                    v[i] = T::one();
                }
            }
        }
        v
    }
}

impl<T: Clone + Num + One + Zero + FromPrimitive> IntoIterator for IdentityMatrix<T> {
    type Item = T;
    type IntoIter = IdentityMatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        IdentityMatrixIntoIterator { index: 0, mat: self }
    }
}

impl<T> fmt::Display for IdentityMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IdentityMatrix(n: {})", self.n)
    }
}

pub struct IdentityMatrixIntoIterator<T> {
    index: usize,
    mat: IdentityMatrix<T>,
}

impl<T: Clone + Num + One + Zero + FromPrimitive> Iterator for IdentityMatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;
        if self.index < self.mat.rows() * self.mat.cols() {
            if self.index % (self.mat.cols()+1) == 0 {
                result = Some(One::one())
            } else {
                result = Some(Zero::zero())
            }
        }
        self.index += 1;
        result
    }
}

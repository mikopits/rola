use std::ops::{Add, Mul};

use num::{Num, Unsigned, Zero, ToPrimitive, FromPrimitive};
//use scoped_threadpool::Pool;

use matrix::Matrix;

/// A DenseMatrix is a matrix that contains many data points in
/// non-concentrated areas. Holds m times n numbers in memory.
#[derive(Debug)]
pub struct DenseMatrix<D, T> {
    m: D,
    n: D,
    mat: Vec<T>,
}

impl<D: Unsigned, T: Clone + Num> DenseMatrix<D, T> {
    /// Create a new DenseMatrix given dimensions and its data (from left to
    /// right, top to bottom).
    #[inline]
    pub fn new(mat: &[Vec<T>]) -> ::Result<DenseMatrix<D, T>>
        where D: FromPrimitive,
    {
        let m = mat.len();
        if m == 0 { return Err(::Error::InvalidDimensions) };

        let n = match mat.first() {
            Some(row) => row.len(),
            None => return Err(::Error::InvalidDimensions),
        };

        let mut flat_mat = Vec::with_capacity(m*n);

        for row in mat {
            if row.len() != n { return Err(::Error::InvalidDimensions) };
            for a in row {
                flat_mat.push(a.to_owned());
            }
        }

        Ok(DenseMatrix {
            m: D::from_usize(m).unwrap(),
            n: D::from_usize(n).unwrap(),
            mat: flat_mat,
        })
    }

    /// Create a new dense matrix of zeros given the matrix dimensions m and n.
    #[inline]
    pub fn zeros(m: D, n: D) -> DenseMatrix<D, T>
        where T: Zero + FromPrimitive,
              D: ToPrimitive,
    {
        let mdim = m.to_usize().unwrap();
        let ndim = n.to_usize().unwrap();
        DenseMatrix{
            m,
            n,
            mat: vec![T::from_usize(0).unwrap(); mdim*ndim],
        }
    }

    /// Create a new identity matrix given its dimension.
    #[inline]
    pub fn identity(n: D) -> DenseMatrix<D, T>
        where T: FromPrimitive,
              D: ToPrimitive + Clone,
    {
        let dim = n.to_usize().unwrap();
        let mut mat = vec![T::from_usize(0 as usize).unwrap(); dim*dim];
        for i in 0..dim {
            mat[i*dim + i] = T::from_usize(1 as usize).unwrap();
        }

        DenseMatrix{ m: n.clone(), n, mat }
    }
}

impl<D: 'static + Clone + Ord + Unsigned + ToPrimitive + FromPrimitive,
     T: 'static + Clone + Num + ToPrimitive + FromPrimitive>
     Matrix for DenseMatrix<D, T> {
    #[inline]
    fn is_hermitian(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_symmetric_positive_definite(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_symmetric_positive_semi_definite(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_skew_hermitian(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_skew_symmetric(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_unitary(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_orthogonal(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_lower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_unilower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_strictly_lower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_lower_hessenberg(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_upper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_uniupper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_strictly_upper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_upper_hessenberg(&self) -> bool {
        // TODO stub
        return false
    }

    #[inline]
    fn is_identity(&self) -> bool {
        // TODO stub
        return false
    }

    // FIXME: don't need to allocate more memory
    #[inline]
    fn trace(&self) -> isize {
        let mut trace = 0;
        let mut i = 0;
        let ndim = self.n.to_usize().unwrap();
        let mdim = self.m.to_usize().unwrap();
        while i < ndim * mdim {
            trace += self.mat[i].to_isize().unwrap();
            i += ndim + 1;
        }
        trace
    }

    #[inline]
    fn transpose(&self) -> Self {
        // TODO stub
        return Self::identity(D::from_usize(2 as usize).unwrap())
    }

    /// Get the dimensions of the matrix.
    #[inline]
    fn dims(&self) -> (usize, usize) {
        (self.m.to_usize().unwrap(), self.n.to_usize().unwrap())
    }
}

impl<D: Unsigned, T: Num> PartialEq for DenseMatrix<D, T> {
    /*fn eq<M>(&self, other: &M) -> bool
        where M: Matrix + Sized,
    {
        // TODO: equality with any matrix type
    }*/

    fn eq(&self, other: &DenseMatrix<D, T>) -> bool {
        if self.m != other.m || self.n != other.n {
            return false
        }

        self.mat == other.mat
    }
}

impl<D: Unsigned, T: Clone + Num> Add for DenseMatrix<D, T> {
    type Output = DenseMatrix<D, T>;

    fn add(self, other: DenseMatrix<D, T>) -> DenseMatrix<D, T> {
        let mut mat = Vec::new();

        for i in 0..self.mat.len() {
            mat.push(self.mat[i].clone() + other.mat[i].clone());
        }

        DenseMatrix {
            m: self.m,
            n: self.n,
            mat
        }
    }
}

impl<D: Unsigned, T: Num> Mul for DenseMatrix<D, T> {
    type Output = DenseMatrix<D, T>;

    fn mul(self, _rhs: DenseMatrix<D, T>) -> DenseMatrix<D, T> {
        // TODO: stub
        self
    }
}

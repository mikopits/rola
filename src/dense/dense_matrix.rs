use std::fmt;
use std::cell::Cell;

use ::{FromPrimitive, Num, ToPrimitive, Zero};
use ::{Matrix, ReadOrder};

/// A DenseMatrix is a matrix that contains many data points in
/// non-concentrated areas. Holds m times n numbers in memory.
///
/// We use row major ordering by default.
#[derive(Clone, Debug)]
pub struct DenseMatrix<T> where T: Copy {
    pub read_order: ReadOrder,
    m: usize,
    n: usize,
    pub mat: Vec<Cell<T>>,
}

impl<T: Clone + Copy + Num> DenseMatrix<T> {
    /// Create a new DenseMatrix given dimensions and its data in a slice of
    /// matrix rows in vec form.
    #[inline]
    pub fn new(mat: &[Vec<T>]) -> ::Result<DenseMatrix<T>> {
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
                flat_mat.push(Cell::new(a.to_owned()));
            }
        }

        Ok(DenseMatrix {
            read_order: ReadOrder::default(),
            m, n,
            mat: flat_mat })
    }

    /// Create a new dense matrix from a Vec, its dimensions, and an
    /// `Option<ReadOrder>`.
    /// If no ReadOrder is provided, then assumes `ReadOrder::RowMajor`.
    #[inline]
    pub fn from_vec(mat: Vec<T>,
                    m: usize,
                    n: usize,
                    read_order: Option<ReadOrder>) -> ::Result<DenseMatrix<T>>
    {
        if m*n != mat.len() {
            return Err(::Error::InvalidDimensions)
        }

        Ok(DenseMatrix {
            read_order: match read_order {
                Some(ro) => ro,
                None => ReadOrder::RowMajor,
            },
            m, n,
            mat: mat.iter().map(|&a| Cell::new(a)).collect(),
        })
    }

    /// Create a new dense matrix of zeros given the matrix dimensions m and n.
    #[inline]
    pub fn zeros(m: usize, n: usize) -> DenseMatrix<T>
        where T: Zero + FromPrimitive,
    {
        DenseMatrix{
            read_order: ReadOrder::default(),
            m, n,
            mat: vec![Cell::new(T::from_usize(0 as usize).unwrap()); m*n],
        }
    }

    /// Create a new identity matrix given its dimension.
    #[inline]
    pub fn identity(n: usize) -> DenseMatrix<T>
        where T: FromPrimitive,
    {
        let mut mat = vec![Cell::new(T::from_usize(0 as usize).unwrap()); n*n];
        for i in 0..n {
            mat[i*n + i] = Cell::new(T::from_usize(1 as usize).unwrap());
        }

        DenseMatrix{
            read_order: ReadOrder::default(),
            m: n, n, mat }
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

impl<T: Clone + Copy + Num + Zero + ToPrimitive + FromPrimitive>
    Matrix<T> for DenseMatrix<T>
{
    fn is_symmetric(&self) -> bool {
        if !self.is_square() { return false }
        for i in 1..self.rows() {
            for j in 0..i {
                if self.element(i, j) != self.element(j, i) {
                    return false
                }
            }
        }
        true
    }

    fn is_orthogonal(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_diagonal(&self) -> bool {
        for i in 0..self.m {
            for j in 0..self.n {
                if i != j {
                    if self.element(i, j).expect("DenseMatrix::is_diagonal")
                        != T::zero() { return false }
                }
            }
        }
        true
    }

    fn is_lower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_unilower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_strictly_lower_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_lower_hessenberg(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_upper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_uniupper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_strictly_upper_triangular(&self) -> bool {
        // TODO stub
        return false
    }

    fn is_upper_hessenberg(&self) -> bool {
        // TODO stub
        return false
    }

    fn trace(&self) -> T {
        let mut trace = T::zero();
        let mut i = 0;
        while i < self.n * self.m {
            trace = trace +
                self.mat.get(i).expect("DenseMatrix::trace").get();
            i += self.n + 1;
        }
        trace
    }

    // TODO: For now, never do in-memory swapping.
    fn transpose(self) -> Self {
        self.flip_read_order()
    }

    /// Get the row dimension of the matrix.
    fn rows(&self) -> usize {
        match self.read_order {
            ReadOrder::RowMajor => self.m,
            ReadOrder::ColMajor => self.n,
        }
    }

    /// Get the column dimension of the matrix.
    fn cols(&self) -> usize {
        match self.read_order {
            ReadOrder::RowMajor => self.n,
            ReadOrder::ColMajor => self.m,
        }
    }

    /// Get a matrix element at m, n.
    fn element(&self, i: usize, j: usize) -> Option<T> {
        match self.read_order {
            ReadOrder::RowMajor => {
                Some(self.mat.get(self.n*i + j)
                    .expect("DenseMatrix::element")
                    .get())
            },
            ReadOrder::ColMajor => {
                Some(self.mat.get(self.m*j + i)
                    .expect("DenseMatrix::element")
                    .get())
            },
        }
    }

    /// Get the elements of the matrix as a Vec.
    /// Returns the elements in row major order.
    fn elements(&self) -> Vec<T> {
        self.mat.iter().map(|c| c.get()).collect()
    }
}

impl<T: Clone + Copy + Num + ToPrimitive + FromPrimitive>
    IntoIterator for DenseMatrix<T> {
    type Item = T;
    type IntoIter = DenseMatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DenseMatrixIntoIterator { mat: self, index: 0 }
    }
}

impl<T: Copy> fmt::Display for DenseMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DenseMatrix(m: {}, n: {})", self.m, self.n)
    }
}

pub struct DenseMatrixIntoIterator<T> where T: Copy {
    mat: DenseMatrix<T>,
    index: usize,
}

impl<T: Clone + Copy + Num + ToPrimitive + FromPrimitive>
    Iterator for DenseMatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;
        if self.index < self.mat.rows() * self.mat.cols() {
            match self.mat.read_order {
                ReadOrder::RowMajor => {
                    result = Some(self.mat.mat.get(self.index /
                                  self.mat.n + self.index % self.mat.n)
                                  .expect("DenseMatrixIntoIterator::next")
                                  .get());
                },
                ReadOrder::ColMajor => {
                    result = Some(self.mat.mat.get(self.index /
                                  self.mat.m + self.index % self.mat.m)
                                  .expect("DenseMatrixIntoIterator::next")
                                  .get());
                },
            }
        }
        self.index += 1;
        result
    }
}

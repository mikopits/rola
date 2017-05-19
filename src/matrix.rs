use std::cmp;

pub trait Matrix<T>: Sized where T: ::Num + Clone {

    // TODO: allow num::Complex.
    ///// Check if the matrix is Hermitian.
    //fn is_hermitian(&self) -> bool;

    /// Check if the matrix is symmetric.
    #[inline]
    fn is_symmetric(&self) -> bool;

    /*/// Check if the matrix is symmetric positive definite.
    fn is_symmetric_positive_definite(&self) -> bool;

    /// Check if the matrix is symmetric positive semi-definite.
    fn is_symmetric_positive_semi_definite(&self) -> bool;

    /// Check if the matrix is skew Hermitian.
    fn is_skew_hermitian(&self) -> bool;

    /// Check if the matrix is skew Symmetric.
    fn is_skew_symmetric(&self) -> bool;

    // Needs num::Complex
    /// Check if the matrix is unitary.
    fn is_unitary(&self) -> bool;
    */

    /// Check if the matrix is orthogonal.
    #[inline]
    fn is_orthogonal(&self) -> bool;

    /// Check if the matrix is diagonal.
    #[inline]
    fn is_diagonal(&self) -> bool;

    /// Check if the matrix is lower triangular.
    #[inline]
    fn is_lower_triangular(&self) -> bool;

    /// Check if the matrix is unilower triangular.
    #[inline]
    fn is_unilower_triangular(&self) -> bool;

    //fn is_atomic_lower_triangular(&self) -> bool;

    /// Check if the matrix is strictly lower triangular.
    #[inline]
    fn is_strictly_lower_triangular(&self) -> bool;

    /// Check if the matrix is lower Hessenberg.
    #[inline]
    fn is_lower_hessenberg(&self) -> bool;

    /// Check if the matrix is upper triangular.
    #[inline]
    fn is_upper_triangular(&self) -> bool;

    /// Check if the matrix is uniupper triangular.
    #[inline]
    fn is_uniupper_triangular(&self) -> bool;

    //fn is_atomic_upper_triangular(&self) -> bool;

    /// Check if the matrix is strictly upper triangular.
    #[inline]
    fn is_strictly_upper_triangular(&self) -> bool;

    /// Check if the matrix is upper Hessenberg.
    #[inline]
    fn is_upper_hessenberg(&self) -> bool;

    /// Compute the trace of the matrix.
    #[inline]
    fn trace(&self) -> T;

    /// Get the transpose of the matrix.
    #[inline]
    fn transpose(self) -> Self;

    /// Get the number of rows `m` in the matrix.
    #[inline]
    fn rows(&self) -> usize;

    /// Get the number of columns `n` in the matrix.
    #[inline]
    fn cols(&self) -> usize;

    /// Get the matrix element at (i, j).
    #[inline]
    fn get(&self, i: usize, j: usize) -> Option<T>;

    /// Set the matrix element at (i, j).
    #[inline]
    fn set(&self, i: usize, j: usize, val: T) -> Option<T>;

    /// Get the matrix elements as a Vec.
    #[inline]
    fn elements(&self) -> Vec<T>;

    /// Returns true if matrix dimensions are equal.
    #[inline]
    fn is_square(&self) -> bool {
        let (m, n) = self.dims(); m == n
    }

    /// Get the matrix dimensions.
    #[inline]
    fn dims(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Get the matrix diagonals.
    #[inline]
    fn diags(&self) -> Vec<T> {
        let (m, n) = self.dims();
        let min = cmp::min(m, n);
        let mut diags = Vec::with_capacity(min);
        for i in 0..min {
            diags.push(self.get(i, i).unwrap());
        }
        diags
    }
}

/// A ReadOrder tells a matrix how it should interpret its data.
#[derive(Clone, Debug)]
pub enum ReadOrder {
    RowMajor,
    ColMajor,
}

impl Default for ReadOrder {
    fn default() -> ReadOrder { ReadOrder::RowMajor }
}

/// Flags are also used to give guarantees. (ie. a matrix flagged with
/// Flag::Invertible will never actually check if it is invertible.)
pub enum Flag {
    Symmetric,
    Orthogonal,
    Diagonal,
    Invertible,
}

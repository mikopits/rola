use std::any::Any;

pub trait Matrix: Any + PartialEq {
    /// Check if the matrix is Hermitian.
    fn is_hermitian(&self) -> bool;

    /// Check if the matrix is symmetric.
    fn is_symmetric(&self) -> bool;

    /// Check if the matrix is symmetric positive definite.
    fn is_symmetric_positive_definite(&self) -> bool;

    /// Check if the matrix is symmetric positive semi-definite.
    fn is_symmetric_positive_semi_definite(&self) -> bool;

    /// Check if the matrix is skew Hermitian.
    fn is_skew_hermitian(&self) -> bool;

    /// Check if the matrix is skew Symmetric.
    fn is_skew_symmetric(&self) -> bool;

    /// Check if the matrix is unitary.
    fn is_unitary(&self) -> bool;

    /// Check if the matrix is orthogonal.
    fn is_orthogonal(&self) -> bool;

    /// Check if the matrix is diagonal.
    fn is_diagonal(&self) -> bool;

    /// Check if the matrix is lower triangular.
    fn is_lower_triangular(&self) -> bool;

    /// Check if the matrix is unilower triangular.
    fn is_unilower_triangular(&self) -> bool;

    //fn is_atomic_lower_triangular(&self) -> bool;

    /// Check if the matrix is strictly lower triangular.
    fn is_strictly_lower_triangular(&self) -> bool;

    /// Check if the matrix is lower Hessenberg.
    fn is_lower_hessenberg(&self) -> bool;

    /// Check if the matrix is upper triangular.
    fn is_upper_triangular(&self) -> bool;

    /// Check if the matrix is uniupper triangular.
    fn is_uniupper_triangular(&self) -> bool;

    //fn is_atomic_upper_triangular(&self) -> bool;

    /// Check if the matrix is strictly upper triangular.
    fn is_strictly_upper_triangular(&self) -> bool;

    /// Check if the matrix is upper Hessenberg.
    fn is_upper_hessenberg(&self) -> bool;

    /// Check if the matrix is the identity matrix.
    fn is_identity(&self) -> bool;

    /// Compute the trace of the matrix.
    fn trace(&self) -> isize;

    /// Get the transpose of the matrix;
    fn transpose(&self) -> Self;

    /// Get the dimensions of the matrix;
    fn dims(&self) -> (usize, usize);
}

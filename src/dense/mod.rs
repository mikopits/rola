pub use self::dense_matrix::DenseMatrix;
pub use self::dense_vector::{DenseColumn, DenseRow};

mod dense_matrix;
mod dense_vector;

#[cfg(test)]
mod tests {
    use dense::DenseMatrix;

    #[allow(non_snake_case)]
    #[test]
    fn test_zero() {
        let Z = DenseMatrix::zeros(5, 4);
        let A = DenseMatrix::new(&vec![vec![0; 4]; 5]).unwrap();
        assert_eq!(Z, A);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_identity() {
        let I = DenseMatrix::identity(5);
        let mat = vec![vec![1, 0, 0, 0, 0],
                       vec![0, 1, 0, 0, 0],
                       vec![0, 0, 1, 0, 0],
                       vec![0, 0, 0, 1, 0],
                       vec![0, 0, 0, 0, 1]];
        let A = DenseMatrix::new(&mat).unwrap();
        assert_eq!(I, A);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_trace() {
        use matrix::Matrix;

        let mat = vec![vec![1, 6, 0],
                       vec![0, 2, 0],
                       vec![4, 0, 3],
                       vec![0, 5, 0]];
        let A = DenseMatrix::new(&mat).unwrap();
        assert_eq!(A.trace(), 6);

        let mat2 = vec![vec![1, 9, 9, 9],
                        vec![9, 2, 9, 9],
                        vec![9, 9, 3, 9]];
        let B = DenseMatrix::new(&mat2).unwrap();
        assert_eq!(B.trace(), 6);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_add() {
        let v = vec![vec![1, 2],
                     vec![3, 4]];
        let A = DenseMatrix::new(&v).unwrap();
        let B = DenseMatrix::new(&v).unwrap();
        let vf = vec![vec![2, 4],
                      vec![6, 8]];
        let A_plus_B = DenseMatrix::new(&vf).unwrap();
        assert_eq!(A+B, A_plus_B);
    }
}

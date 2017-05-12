use num::{Num, One, Zero, FromPrimitive, ToPrimitive};

use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};

impl<T: Clone + Num + ToPrimitive + FromPrimitive>
    PartialEq for ZeroMatrix<T>
{
    fn eq(&self, other: &ZeroMatrix<T>) -> bool {
        self.dims() == other.dims()
    }
}

impl<T: Clone + Num + ToPrimitive + FromPrimitive>
    PartialEq for IdentityMatrix<T>
{
    fn eq(&self, other: &IdentityMatrix<T>) -> bool {
        self.dims() == other.dims()
    }
}

impl<T: Clone + Num + ToPrimitive + FromPrimitive>
    PartialEq for DenseMatrix<T>
{
    fn eq(&self, other: &DenseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        self.mat == other.mat
    }
}

impl<T: Clone + Num + One + ToPrimitive + FromPrimitive>
    PartialEq<IdentityMatrix<T>> for DenseMatrix<T>
{
    fn eq(&self, other: &IdentityMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        if !self.is_diagonal() { return false }
        for a in self.diags() {
            if a != T::one() { return false }
        }
        true
    }
}

impl<T: Clone + Num + One + ToPrimitive + FromPrimitive>
    PartialEq<DenseMatrix<T>> for IdentityMatrix<T>
{
    fn eq(&self, other: &DenseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        if !other.is_diagonal() { return false }
        for a in other.diags() {
            if a != T::one() { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<ZeroMatrix<T>> for DenseMatrix<T>
{
    fn eq(&self, other: &ZeroMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        for a in &self.mat {
            if a != &T::zero() { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<DenseMatrix<T>> for ZeroMatrix<T>
{
    fn eq(&self, other: &DenseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        for a in &other.mat {
            if a != &T::zero() { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<IdentityMatrix<T>> for ZeroMatrix<T>
{
    fn eq(&self, _other: &IdentityMatrix<T>) -> bool { false }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<ZeroMatrix<T>> for IdentityMatrix<T>
{
    fn eq(&self, _other: &ZeroMatrix<T>) -> bool { false }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq for SparseMatrix<T>
{
    fn eq(&self, other: &SparseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        self.mat == other.mat
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<DenseMatrix<T>> for SparseMatrix<T>
{
    fn eq(&self, other: &DenseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self.element(i, j) != other.element(i, j) {
                    return false
                }
            }
        }
        true
    }
}

impl<T: Clone + Num + One + Zero + ToPrimitive + FromPrimitive>
    PartialEq<IdentityMatrix<T>> for SparseMatrix<T>
{
    fn eq(&self, other: &IdentityMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        if !self.is_diagonal() { return false }
        for a in self.diags() {
            if a != T::one() { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<ZeroMatrix<T>> for SparseMatrix<T>
{
    fn eq(&self, other: &ZeroMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        for a in self.elements() {
            if a != Zero::zero() { return false }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use std::collections::HashMap;
    use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};

    #[test]
    fn test_eq_dense_identity() {
        let mat1 = vec![vec![1, 0, 0],
                        vec![0, 1, 0],
                        vec![0, 0, 1]];
        let mat2 = vec![vec![1, 1, 0],
                        vec![0, 1, 0],
                        vec![0, 0, 1]];
        let A1 = DenseMatrix::new(&mat1).unwrap();
        assert!(A1.is_diagonal());
        let A2 = DenseMatrix::new(&mat2).unwrap();
        assert!(!A2.is_diagonal());
        let I = IdentityMatrix::new(3);
        assert_eq!(A1, I);
        assert_eq!(I, A1);
        assert_ne!(A2, I);
        assert_ne!(I, A2);
    }

    #[test]
    fn test_eq_dense_zero() {
        let mat1 = vec![vec![0, 0, 0],
                        vec![0, 0, 0],
                        vec![0, 0, 0]];
        let mat2 = vec![vec![0, 0, 0],
                        vec![0, 1, 0],
                        vec![0, 0, 0]];
        let A1 = DenseMatrix::new(&mat1).unwrap();
        let A2 = DenseMatrix::new(&mat2).unwrap();
        let Z = ZeroMatrix::new(3, 3);
        assert_eq!(A1, Z);
        assert_eq!(Z, A1);
        assert_ne!(A2, Z);
        assert_ne!(Z, A2);
    }

    #[test]
    fn test_eq_zero_zero() {
        let Z1: ZeroMatrix<usize> = ZeroMatrix::new(4, 20);
        let Z2 = ZeroMatrix::new(4, 20);
        let Z3 = ZeroMatrix::new(20, 4);
        assert_eq!(Z1, Z2);
        assert_eq!(Z2, Z1);
        assert_ne!(Z1, Z3);
        assert_ne!(Z3, Z1);
    }

    #[test]
    fn test_eq_ident_ident() {
        let I1: IdentityMatrix<usize> = IdentityMatrix::new(10000000);
        let I2 = IdentityMatrix::new(10000000);
        let I3 = IdentityMatrix::new(10000001);
        assert_eq!(I1, I2);
        assert_eq!(I2, I1);
        assert_ne!(I1, I3);
        assert_ne!(I3, I1);
    }

    #[test]
    fn test_eq_ident_zero() {
        let I: IdentityMatrix<usize> = IdentityMatrix::new(100);
        let Z = ZeroMatrix::new(100, 100);
        assert_ne!(I, Z);
        assert_ne!(Z, I);
    }

    #[test]
    fn test_sparse_eq() {
        let mat_zero: HashMap<(usize, usize), usize> = HashMap::new();
        let A_z = SparseMatrix::new(mat_zero, 100, 200);
        let Z = ZeroMatrix::new(100, 200);
        assert_eq!(A_z, Z);
        let mut mat_eye = HashMap::new();
        for i in 0..100 {
            mat_eye.insert((i, i), 1);
        }
        let A_i = SparseMatrix::new(mat_eye, 100, 100);
        let I = IdentityMatrix::new(100);
        assert_eq!(A_i, I);
        let mut mat_sparse = HashMap::new();
        for j in 0..5 {
            for i in 0..5 {
                mat_sparse.insert((i, j), i+j);
            }
        }
        let mat_dense = vec![vec![0, 1, 2, 3, 4],
                             vec![1, 2, 3, 4, 5],
                             vec![2, 3, 4, 5, 6],
                             vec![3, 4, 5, 6, 7],
                             vec![4, 5, 6, 7, 8]];
        let A_s = SparseMatrix::new(mat_sparse, 5, 5);
        let A_d = DenseMatrix::new(&mat_dense).unwrap();
        assert_eq!(A_s, A_d);
    }
}

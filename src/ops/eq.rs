use num::{Num, One, Zero, FromPrimitive, ToPrimitive};

use ::{DenseMatrix, IdentityMatrix, Matrix, ZeroMatrix};

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

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use ::{DenseMatrix, IdentityMatrix, Matrix, ZeroMatrix};

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
}

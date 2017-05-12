use ::{FromPrimitive, Num, One, ToPrimitive, Zero};
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
    where T: Copy,
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

impl<T: Clone + Num + One + ToPrimitive + FromPrimitive>
    PartialEq<IdentityMatrix<T>> for DenseMatrix<T>
    where T: Copy,
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
    where T: Copy,
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
    where T: Copy,
{
    fn eq(&self, other: &ZeroMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        self.elements().iter().all(|&a| a == T::zero())
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<DenseMatrix<T>> for ZeroMatrix<T>
    where T: Copy,
{
    fn eq(&self, other: &DenseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        other.elements().iter().all(|&a| a == T::zero())
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<IdentityMatrix<T>> for ZeroMatrix<T>
{
    fn eq(&self, _other: &IdentityMatrix<T>) -> bool { false }
}

impl<T: Clone + Copy + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<SparseMatrix<T>> for ZeroMatrix<T>
{
    fn eq(&self, other: &SparseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        other.mat.borrow().values().all(|ref c| c.get() == T::zero())
    }
}

impl<T: Clone + Copy + Num + One + ToPrimitive + FromPrimitive>
    PartialEq<SparseMatrix<T>> for IdentityMatrix<T>
{
    fn eq(&self, other: &SparseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        if !other.is_diagonal() { return false }
        for i in 0..self.rows() {
            if other.element(i, i).unwrap() != One::one() { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<ZeroMatrix<T>> for IdentityMatrix<T>
{
    fn eq(&self, _other: &ZeroMatrix<T>) -> bool { false }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq for SparseMatrix<T>
    where T: Copy,
{
    fn eq(&self, other: &SparseMatrix<T>) -> bool {
        if self.dims() != other.dims() { return false }
        if self.mat.borrow().keys().len() != other.mat.borrow().keys().len() {
            return false
        }
        for (&(i1, j1), &(i2, j2)) in self.mat.borrow().keys()
            .zip(other.mat.borrow().keys()) {
            if self.element(i1, j1) != other.element(i2, j2) { return false }
        }
        true
    }
}

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<DenseMatrix<T>> for SparseMatrix<T>
    where T: Copy,
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

impl<T: Clone + Num + Zero + ToPrimitive + FromPrimitive>
    PartialEq<SparseMatrix<T>> for DenseMatrix<T>
    where T: Copy,
{
    fn eq(&self, other: &SparseMatrix<T>) -> bool {
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
    where T: Copy,
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

impl<T: Clone + Copy + Num + Zero + ToPrimitive + FromPrimitive>
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
    use ::{IdentityMatrix, Matrix, ZeroMatrix};

    #[test]
    fn test_eq_dense_identity() {
        let A1 = dense![1, 0, 0; 0, 1, 0; 0, 0, 1].unwrap();
        assert!(A1.is_diagonal());
        let A2 = dense![1, 1, 0; 0, 1, 0; 0, 0, 1].unwrap();
        assert!(!A2.is_diagonal());
        let I = eye!(3);
        assert_eq!(A1, I);
        assert_eq!(I, A1);
        assert_ne!(A2, I);
        assert_ne!(I, A2);
    }

    #[test]
    fn test_eq_dense_zero() {
        let A1 = dense![0, 0, 0; 0, 0, 0; 0, 0, 0].unwrap();
        let A2 = dense![0, 0, 0; 0, 1, 0; 0, 0, 0].unwrap();
        let Z = zeros!(3, 3);
        assert_eq!(A1, Z);
        assert_eq!(Z, A1);
        assert_ne!(A2, Z);
        assert_ne!(Z, A2);
    }

    #[test]
    fn test_eq_zero_zero() {
        let Z1: ZeroMatrix<usize> = zeros!(4, 20);
        let Z2 = zeros!(4, 20);
        let Z3 = zeros!(20, 4);
        assert_eq!(Z1, Z2);
        assert_eq!(Z2, Z1);
        assert_ne!(Z1, Z3);
        assert_ne!(Z3, Z1);
    }

    #[test]
    fn test_eq_ident_ident() {
        let I1: IdentityMatrix<usize> = eye!(10000000);
        let I2 = eye!(10000000);
        let I3 = eye!(10000001);
        assert_eq!(I1, I2);
        assert_eq!(I2, I1);
        assert_ne!(I1, I3);
        assert_ne!(I3, I1);
    }

    #[test]
    fn test_eq_ident_zero() {
        let I: IdentityMatrix<usize> = eye!(100);
        let Z = zeros!(100, 100);
        assert_ne!(I, Z);
        assert_ne!(Z, I);
    }

    #[test]
    fn test_sparse_eq() {
        let v_0: Vec<(usize, usize, usize)> = Vec::new();
        let A_z = sparse![v_0; 100, 200];
        let Z = zeros!(100, 200);
        assert_eq!(A_z, Z);
        assert_eq!(Z, A_z);

        let mut mat_eye = Vec::new();
        for i in 0..100 {
            mat_eye.push((i, i, 1));
        }
        let A_i = sparse![mat_eye; 100, 100];
        let I = eye!(100);
        assert_eq!(A_i, I);
        assert_eq!(I, A_i);

        let mut mat_sparse = Vec::new();
        for j in 0..5 {
            for i in 0..5 {
                mat_sparse.push((i, j, i+j));
            }
        }
        let A_d = dense![0, 1, 2, 3, 4;
                         1, 2, 3, 4, 5;
                         2, 3, 4, 5, 6;
                         3, 4, 5, 6, 7;
                         4, 5, 6, 7, 8].unwrap();
        let A_s = sparse![mat_sparse; 5, 5];
        assert_eq!(A_s, A_d);
        assert_eq!(A_d, A_s);
    }
}

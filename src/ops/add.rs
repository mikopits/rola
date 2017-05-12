use std::ops::Add;

use num::{FromPrimitive, Num, ToPrimitive};

use ::{DenseMatrix, IdentityMatrix, Matrix, ZeroMatrix};

static ADD_DIM_ERROR: &str = "Cannot add matrices of given dimensions";

macro_rules! check_add_dims {
    ($self:expr, $other:expr) => (
        if $self.dims() != $other.dims() {
            panic!("{}: lhs={} rhs={}", ADD_DIM_ERROR, $self, $other)
        }
    )
}

macro_rules! zero_add_impl {
    ($($t:ty)*) => ($(
        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Add<$t> for ZeroMatrix<T>
        {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t {
                check_add_dims! { self, other }
                other
            }
        }
    )*)
}

zero_add_impl! { DenseMatrix<T> IdentityMatrix<T> ZeroMatrix<T> }

impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for DenseMatrix<T> {
    type Output = DenseMatrix<T>;

    fn add(self, other: DenseMatrix<T>) -> DenseMatrix<T> {
        check_add_dims! { self, other }
        let mut mat = Vec::new();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                mat.push(self.element(i, j).expect("DenseMatrix::add") +
                         other.element(i, j).expect("DenseMatrix::add"));
            }
        }
        DenseMatrix::from_vec(mat, self.rows(), self.cols(), None)
    }
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Add<IdentityMatrix<T>> for DenseMatrix<T>
{
    type Output = DenseMatrix<T>;

    fn add(self, other: IdentityMatrix<T>) -> DenseMatrix<T> {
        check_add_dims! {self, other}
        let mut mat = Vec::with_capacity(self.rows()*self.cols());
        // Guaranteed to be square.
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if i == j {
                    mat.push(self.element(i, j).expect("DenseMatrix::add")
                             + T::one());
                } else {
                    mat.push(self.element(i, j).expect("DenseMatrix::add"));
                }
            }
        }
        DenseMatrix::from_vec(mat,
                              self.rows(),
                              self.cols(),
                              Some(self.read_order))
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use ::{DenseMatrix, IdentityMatrix, ZeroMatrix};

    #[test]
    fn test_good_zero_add() {
        let Z1: ZeroMatrix<usize> = ZeroMatrix::new(4, 4);
        let Z2 = ZeroMatrix::new(4, 4);
        let I = IdentityMatrix::new(4);
        let mat = vec![vec![ 1, 2, 3, 4],
                       vec![ 5, 6, 7, 8],
                       vec![ 9,10,11,12],
                       vec![13,14,15,16]];
        let A = DenseMatrix::new(&mat).unwrap();
        assert_eq!(Z1.clone() + Z2.clone(), Z2);
        assert_eq!(Z1.clone() + I.clone(), I);
        assert_eq!(Z1 + A.clone(), A);
    }

    #[test]
    #[should_panic]
    fn test_bad_zero_add() {
        let Z1: ZeroMatrix<usize> = ZeroMatrix::new(21, 69);
        let Z2 = ZeroMatrix::new(21, 42);
        let _panic = Z1 + Z2;
    }

    #[test]
    fn test_dense_ident_add() {
        let mat1 = vec![vec![ 1, 2, 3, 4],
                        vec![ 5, 6, 7, 8],
                        vec![ 9,10,11,12],
                        vec![13,14,15,16]];
        let mat2 = vec![vec![ 2, 2, 3, 4],
                        vec![ 5, 7, 7, 8],
                        vec![ 9,10,12,12],
                        vec![13,14,15,17]];
        let A1 = DenseMatrix::new(&mat1).unwrap();
        let A2 = DenseMatrix::new(&mat2).unwrap();
        let I = IdentityMatrix::new(4);
        assert_eq!(A1+I, A2);
    }
}

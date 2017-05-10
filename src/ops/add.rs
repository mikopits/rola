use std::ops::Add;

use num::{FromPrimitive, Num, ToPrimitive};

use ::{DenseMatrix, IdentityMatrix, Matrix, ZeroMatrix};

static ADD_DIM_ERROR: &str = "Cannot add matrices of given dimensions";

macro_rules! zero_add_impl {
    ($($t:ty)*) => ($(
        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Add<$t> for ZeroMatrix<T>
        {
            type Output = $t;

            #[inline]
            fn add(self, rhs: $t) -> $t {
                if self.dims() != rhs.dims() {
                    panic!("{}: lhs={} rhs={}", ADD_DIM_ERROR, self, rhs)
                }
                rhs
            }
        }
    )*)
}

zero_add_impl! { DenseMatrix<T> IdentityMatrix<T> ZeroMatrix<T> }

impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for DenseMatrix<T> {
    type Output = DenseMatrix<T>;

    fn add(self, other: DenseMatrix<T>) -> DenseMatrix<T> {
        let mut mat = Vec::new();

        for i in 0..self.mat.len() {
            mat.push(self.mat[i].clone() + other.mat[i].clone());
        }

        DenseMatrix::from_vec(mat, self.rows(), self.cols(), None)
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
}

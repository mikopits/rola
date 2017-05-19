use std::ops::Mul;

use ::{FromPrimitive, Num, ToPrimitive};
use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};

static MUL_DIM_ERROR: &str = "Cannot multiply matrices of given dimensions";

macro_rules! check_mul_dims {
    ($self:expr, $rhs:expr) => (
        if $self.cols() != $rhs.rows() {
            panic!("{}: lhs={} rhs={}", MUL_DIM_ERROR, $self, $rhs)
        }
    )
}

macro_rules! zero_mul_impl {
    ($($t:ty)*) => ($(
        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Mul<$t> for ZeroMatrix<T>
            where T: Copy,
        {
            type Output = ZeroMatrix<T>;

            #[inline]
            fn mul(self, rhs: $t) -> ZeroMatrix<T> {
                check_mul_dims!(self, rhs);
                ZeroMatrix::new(self.rows(), rhs.cols())
            }
        }

        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Mul<ZeroMatrix<T>> for $t
            where T: Copy,
        {
            type Output = ZeroMatrix<T>;

            #[inline]
            fn mul(self, rhs: ZeroMatrix<T>) -> ZeroMatrix<T> {
                check_mul_dims!(self, rhs);
                ZeroMatrix::new(self.rows(), rhs.cols())
            }
        }
    )*)
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive> Mul for ZeroMatrix<T>
    where T: Copy,
{
    type Output = ZeroMatrix<T>;

    #[inline]
    fn mul(self, rhs: ZeroMatrix<T>) -> ZeroMatrix<T> {
        check_mul_dims!(self, rhs);
        ZeroMatrix::new(self.rows(), rhs.cols())
    }
}

zero_mul_impl! { DenseMatrix<T> SparseMatrix<T> }

macro_rules! ident_mul_impl {
    ($($t:ty)*) => ($(
        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Mul<$t> for IdentityMatrix<T>
            where T: Copy,
        {
            type Output = $t;

            #[inline]
            fn mul(self, rhs: $t) -> $t {
                check_mul_dims!(self, rhs);
                rhs
            }
        }

        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Mul<IdentityMatrix<T>> for $t
            where T: Copy,
        {
            type Output = $t;

            #[inline]
            fn mul(self, rhs: IdentityMatrix<T>) -> $t {
                check_mul_dims!(self, rhs);
                self
            }
        }
    )*)
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Mul for IdentityMatrix<T>
    where T: Copy,
{
    type Output = IdentityMatrix<T>;

    #[inline]
    fn mul(self, _rhs: IdentityMatrix<T>) -> IdentityMatrix<T> {
        self
    }
}


ident_mul_impl! { DenseMatrix<T> SparseMatrix<T> ZeroMatrix<T> }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use ::{IdentityMatrix, Matrix, ZeroMatrix};

    #[test]
    fn test_good_zero_zero_mul() {
        let Z1: ZeroMatrix<usize> = zeros!(69, 21);
        let Z2 = zeros!(21, 42);
        let Z3 = zeros!(69, 42);
        assert_eq!(Z1.clone()*Z2.clone(), Z3);

        let Z4 = Z1.transpose();
        let Z5 = Z3.clone();
        let Z6 = zeros!(21, 42);
        assert_eq!(Z4*Z5, Z6);
    }

    #[test]
    #[should_panic]
    fn test_bad_zero_zero_mul() {
        let Z1: ZeroMatrix<usize> = zeros!(21, 69);
        let Z2 = zeros!(21, 42);
        let _panic = Z1*Z2;
    }

    #[test]
    fn test_ident_mul() {
        let I: IdentityMatrix<usize> = eye!(5);
        let S = sparse!(5, 5);
        for i in 0..5 {
            for j in 0..5 {
                S.set(i, j, 5*i+j);
            }
        }
        let D = dense![0, 1, 2, 3, 4;
                       5, 6, 7, 8, 9;
                      10,11,12,13,14;
                      15,16,17,18,19;
                      20,21,22,23,24].unwrap();
        assert_eq!(I.clone()*S.clone(), S.clone());
        assert_eq!(S.clone()*I.clone(), S);
        assert_eq!(I.clone()*D.clone(), D.clone());
        assert_eq!(D.clone()*I, D);
    }
}

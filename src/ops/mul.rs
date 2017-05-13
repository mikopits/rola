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
                check_mul_dims! { self, rhs }
                ZeroMatrix::new(self.rows(), rhs.cols())
            }
        }
    )*)
}

zero_mul_impl! { DenseMatrix<T> IdentityMatrix<T> SparseMatrix<T> ZeroMatrix<T> }

// TODO: add kernels here
// impl Ops for DenseMatrix<T>
// {
//     fn mul_kernel(){
//
//     }
//
//
//     fn generate_kernel_config(&self, source : &String){
//
//     }
// }
//
// impl Ops for SparseMatrix<T>
// {
//     fn mul_kernel(){
//
//     }
//
//
//     fn generate_kernel_config(&self, source : &String){
//
//     }
// }
//
// impl Ops for ZeroMatrix<T>
// {
//     fn mul_kernel(){
//
//     }
//
//
//     fn generate_kernel_config(&self, source : &String){
//
//     }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use ::{Matrix, ZeroMatrix};

    #[test]
    fn test_good_zero_zero_mul() {
        let Z1: ZeroMatrix<usize> = ZeroMatrix::new(69, 21);
        let Z2 = ZeroMatrix::new(21, 42);
        let Z3 = ZeroMatrix::new(69, 42);
        assert_eq!(Z1.clone()*Z2.clone(), Z3);

        let Z4 = Z1.transpose();
        let Z5 = Z3.clone();
        let Z6 = ZeroMatrix::new(21, 42);
        assert_eq!(Z4*Z5, Z6);
    }

    #[test]
    #[should_panic]
    fn test_bad_zero_zero_mul() {
        let Z1: ZeroMatrix<usize> = ZeroMatrix::new(21, 69);
        let Z2 = ZeroMatrix::new(21, 42);
        let _panic = Z1*Z2;
    }
}

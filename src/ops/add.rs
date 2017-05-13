use std::ops::Add;

use ::{FromPrimitive, Num, ToPrimitive};
use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};



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
            where T: Copy,
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

zero_add_impl! { DenseMatrix<T> IdentityMatrix<T> SparseMatrix<T> ZeroMatrix<T> }

// TODO: add kernels here
// impl Ops for DenseMatrix<T>
// {
//     fn add_kernel(){
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
//     fn add_kernel(){
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
//     fn add_kernel(){
//
//     }
//
//
//     fn generate_kernel_config(&self, source : &String){
//
//     }
// }

impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for DenseMatrix<T>
    where T: Copy,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: DenseMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
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
    where T: Copy,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: IdentityMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows()*self.cols());
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

impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Add<SparseMatrix<T>> for DenseMatrix<T>
    where T: Copy,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: SparseMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows()*self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                mat.push(self.element(i, j).expect("DenseMatrix::add") +
                         other.element(i, j).expect("DenseMatrix::add"));
            }
        }
        DenseMatrix::from_vec(mat,
                              self.rows(),
                              self.cols(),
                              Some(self.read_order))
    }
}


/* FIXME: Cannot infer type of One::one()
impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for IdentityMatrix<T>
    where T: One,
{
    type Output = SparseMatrix<T>;

    fn add(self, other: IdentityMatrix<T>) -> SparseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows());
        for i in 0..self.rows() {
            mat.push((i, i, One::one() + One::one()));
        }
        SparseMatrix::from_tuple(mat, self.rows(), self.cols())
    }
}*/

/*
impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Add<SparseMatrix<T>> for IdentityMatrix<T>
{
    type Output = SparseMatrix<T>;

    fn add(self, other: SparseMatrix<T>) -> SparseMatrix<T> {
        check_add_dims!(self, other);
        for i in 0..self.rows() {
        }
    }
}*/

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

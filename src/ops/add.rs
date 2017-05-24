use std::fmt::Debug;
use std::ops::Add;
use std::ops::Sub;

use ::{FromPrimitive, Num, One, ToPrimitive};
use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};
use ::{Vector};


static ADD_DIM_ERROR: &str = "Cannot add matrices of given dimensions";

macro_rules! check_vec_dims {
    ($self:expr, $other:expr) => (
        if $self.len() != $other.len() {
            panic!("{}: lhs={} rhs={}", ADD_DIM_ERROR, $self, $other)
        }
    )
}

macro_rules! check_add_dims {
    ($self:expr, $other:expr) => (
        if $self.dims() != $other.dims() {
            panic!("{}: lhs={:?} rhs={:?}", ADD_DIM_ERROR, $self, $other)
        }
    )
}

macro_rules! zero_add_impl {
    ($($t:ty)*) => ($(
        impl<T: Clone + Num + FromPrimitive + ToPrimitive>
            Add<$t> for ZeroMatrix<T>
            where T: Copy + Debug,
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


impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for Vector<T>
    where T: Copy,
{
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T>{
        check_vec_dims!(self, other);
        let n = self.len();
        let mut r_vec: Vec<T> = Vec::with_capacity( n );
        unsafe { r_vec.set_len(n); }

        for i in 0..self.len() {
            r_vec[i] = self.vec[i] + other.vec[i];
        }
        Vector::from_vec(r_vec, self.length, None)
    }
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive> Sub for Vector<T>
    where T: Copy,
{
    type Output = Vector<T>;

    fn sub(self, other: Vector<T>) -> Vector<T>{
        check_vec_dims!(self, other);
        let n = self.len();
        let mut r_vec: Vec<T> = Vec::with_capacity( n );
        unsafe { r_vec.set_len(n); }

        for i in 0..self.len() {
            r_vec[i] = self.vec[i] - other.vec[i];
        }
        Vector::from_vec(r_vec, self.length, None)
    }
}


impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for DenseMatrix<T>
    where T: Copy + Debug,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: DenseMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::new();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                mat.push(self.get(i, j).unwrap() + other.get(i, j).unwrap());
            }
        }
        DenseMatrix::from_vec(mat, self.rows(), self.cols(), None).unwrap()
    }
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Add<IdentityMatrix<T>> for DenseMatrix<T>
    where T: Copy + Debug + One,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: IdentityMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows()*self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if i == j {
                    mat.push(self.get(i, j).unwrap() + T::one());
                } else {
                    mat.push(self.get(i, j).unwrap());
                }
            }
        }
        DenseMatrix::from_vec(mat, self.rows(), self.cols(),
                              Some(self.read_order)).unwrap()
    }
}

impl<T: Clone + Num + FromPrimitive + ToPrimitive>
    Add<SparseMatrix<T>> for DenseMatrix<T>
    where T: Copy + Debug,
{
    type Output = DenseMatrix<T>;

    fn add(self, other: SparseMatrix<T>) -> DenseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows()*self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                mat.push(self.get(i, j).unwrap() + other.get(i, j).unwrap());
            }
        }
        DenseMatrix::from_vec(mat, self.rows(), self.cols(),
                              Some(self.read_order)).unwrap()
    }
}


impl<T: Clone + Num + FromPrimitive + ToPrimitive> Add for IdentityMatrix<T>
    where T: Copy + Debug,
{
    type Output = SparseMatrix<T>;

    fn add(self, other: IdentityMatrix<T>) -> SparseMatrix<T> {
        check_add_dims!(self, other);
        let mut mat = Vec::with_capacity(self.rows());
        for i in 0..self.rows() {
            let one = self.get(i, i).unwrap();
            mat.push((i, i, one + one));
        }
        SparseMatrix::from_tuple(mat, self.rows(), self.cols())
    }
}

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
    use ::{Vector};

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

    #[test]
    fn test_vec_add(){
        let n = 1000;
        let vec_0 : Vector<usize> = Vector::zeros(n);
        let vec_ord: Vec<T> = (1..n).collect();
        let vector_ord : Vector<T> = Vector::from_vec(vec_ord);

        let vec_ord_x2: Vec<T> = vec_ord.iter().map(|x| x*2);
        let vector_ord_x2 : Vector<T> = Vector::from_vec(vec_ord_x2);

        assert_eq!( vec_0 + vec_order, vec_order);
        assert_eq!( vec_ord + vec_0, vec_ord);
        assert_eq!( vec_ord + vec_ord, vec_ord_x2);
    }
}

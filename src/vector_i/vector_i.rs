use std::cmp;

use ::{FromPrimitive, Num, One, Zero};
use ::{Matrix, ReadOrder};

struct TransposeVector{
    transpose: true,
    vec: mut Vec<T>
}
/// A Vector is a 1xn || nx1 matrix
/// holds n elements
///
/// We use row major ordering by default.
#[derive(Clone, Debug)]
pub struct Vector<T> where T: Copy {
    transpose: false,
    vec: mut Vec<T>
}

impl<T: Clone + Copy + Num> Vector<T> {

    #[inline]
    pub fn new(vec: &Vec<T>) -> ::Result<Vector<T>> {
        let n = vec.len();
        if n == 0 { return Err(::Error::InvalidDimensions) };

        Ok(Vector {
            transpose: false,
            vec: vec.clone()
        })
    }

    /// Create a new dense matrix from a Vec, its dimensions, and an
    /// `Option<ReadOrder>`.
    /// If no ReadOrder is provided, then assumes `ReadOrder::RowMajor`.
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> Vector<T>
    {
        Vector {
            transpose: false,
            vec: vec.clone(),
        }
    }

    /// Create a vector of 0 of length n
    #[inline]
    pub fn zeros(n: usize) -> Vector<T>
        where T: Zero + FromPrimitive,
    {
        Vector{
            transpose: false,
            mat: vec![0; n],
        }
    }

    /// Create a vector of 1 of length n
    pub fn ones(n: usize) -> Vector<T>
        where T: Zero + FromPrimitive,
    {
        Vector{
            transpose: false,
            mat: vec![1; n],
        }
    }
}

impl<T: Clone + Copy + Num> Matrix<T> for Vector<T> {
    type Transpose = TransposeVector;

    #[inline]
    fn transpose(&self) -> Self::Transpose {
        TransposeVector{
            transpose: true,
            vec: self.vec.clone()
        }
    }

    /// Set an element at position `index`. Returns None if index is out of
    /// bounds.
    #[inline]
    fn set(&self, index: usize, val: T) -> Option<T>{
        if index >= self.len() { return None; }
        if index <= 0 { return None; }
        self.vec[index] = val;

        return val
    }

    /// Get an element at position `index`.
    #[inline]
    fn get(&self, index: usize) -> Option<T> {
        if index >= self.len() { return None; }
        if index <= 0 { return None; }

        self.vec.get(index)
    }

    /// Get the length of the `Vector`.
    #[inline]
    fn len(&self) -> usize
    {
        self.vec.len()
    }

}

#![allow(dead_code)]

extern crate num;
extern crate ocl;

pub use num::traits::*;
pub use num::{Rational, Complex};

pub use self::dense::{DenseMatrix, DenseRow, DenseColumn};
pub use self::error::{Error, Result};
pub use self::matrix::{Matrix, ReadOrder};
pub use self::sparse::{IdentityMatrix, SparseMatrix, ZeroMatrix};
pub use self::vector::Vector;

mod dense;
mod error;
#[macro_use]
mod macros;
mod matrix;
mod ops;
mod sparse;
mod vector;

#![allow(dead_code)]

extern crate num;

pub use num::traits::*;

pub use self::dense::DenseMatrix;
pub use self::error::{Error, Result};
pub use self::matrix::Matrix;
pub use self::sparse::{IdentityMatrix, SparseMatrix, ZeroMatrix};

mod dense;
mod error;
#[macro_use]
mod macros;
mod matrix;
mod sparse;
mod ops;

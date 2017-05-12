#![allow(dead_code)]

extern crate num;

pub use num::traits::*;

pub use dense::DenseMatrix;
pub use error::{Error, Result};
pub use matrix::Matrix;
pub use sparse::{IdentityMatrix, SparseMatrix, ZeroMatrix};

mod dense;
mod error;
mod matrix;
mod sparse;
mod ops;

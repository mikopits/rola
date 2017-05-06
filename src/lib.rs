#![allow(dead_code)]

extern crate num;
extern crate scoped_threadpool;

pub use num::traits::*;

pub use dense::DenseMatrix;
pub use matrix::Matrix;
pub use error::{Error, Result};

mod dense;
mod error;
mod matrix;

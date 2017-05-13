use std::cmp;
use ::{FromPrimitive, Num, ToPrimitive};
use ::{DenseMatrix, IdentityMatrix, Matrix, SparseMatrix, ZeroMatrix};

use ocl::{flags, Platform, Device, Context}

use::{OpenCLHandler}

 ////////////////////////////////////////////////////////////////////////////////
// Container trait for Matrix operation Kernel Strings
// Supported:   add
//              equal
//              subtract
//              multiply
////////////////////////////////////////////////////////////////////////////////

pub trait Ops {

    // configure statics for opencl kkernel program eg. #define ...
    #[inline]
    fn generate_kernel_config(&self, source : &String);

    // Opn Dense Matrices
    #[inline]
    fn add_kernel(&self, source: &String, numeric_type: &String){ println!("{}") }
    #[inline]
    fn eq_kernel(&self, source: &String, numeric_type: &String){ println!("{}") }
    #[inline]
    fn mul_kernel(&self, source: &String, numeric_type: &String){ println!("{}") }

}

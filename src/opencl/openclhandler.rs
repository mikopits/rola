extern crate ocl;

use std::fmt;
use std::ptr;
use std::{mem, thread};

use ocl::ProQue;
use ocl::{flags, Platform, Device, Context, Queue, Program,
        Buffer, Kernel, Event, EventList};

use matrix::{Matrix, /*Flag,*/ ReadOrder};


struct OpenCLHandler {
    platform : Platform,
    device : Device,
    context : Context,
}

// TODO: integrate error handling
//  test
impl OpenCLHandler {

    ////////////////////////////////////////////////////////////////////////////////
    // OpenCLHandler -> handles call from
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new(&self) -> OpenCLHandler
    {
        // only call to self.opencl_components once
        self.platform = Platform::default();
        self.device = Device::first(self.platform);
        self.context = Context::builder()
            .platform(self.platform)
            .devices(self.device.clone())
            .build().unwrap();
    }

    ////////////////////////////////////////////////////////////////////////////////
    // runs given a source string as kernel program
    ////////////////////////////////////////////////////////////////////////////////

    /** runs given a source string as kernel program
    * @param src: kernel program as string
    * @param kernel_fn: function name of kernel program to runs
    * @param kernel_args: array of matrices to read to GPU
    * @param resultant_mat:matrix after operation from GPU to vector object
    */

    pub fn run(&self, src: String, kernel_fn: String, kernel_args: &[Matrix<T>], resultant_mat: &mut Matrix<T>)
    {
        let program = Program::builder()
            .devices(self.device)
            .src(src)
            .build(&self.context).unwrap();

        // TODO Error handling
        let queue = Queue::new(&self.context, self.device, None).unwrap();
        let kernel = Kernel::new( kernel_fn, &program ).unwrap().queue(queue.clone());

        for matrix in kernel_args.iter()
        {
             let dims  = matrix.m * matrix.n;
             let buffer = Buffer::<f32>::builder()
                 .queue(queue.clone())
                 .flags(flags::MEM_READ_WRITE | flags::MEM_COPY_HOST_PTR) // CL::<flags>
                 .dims(dims)
                 .host_data(&matrix.mat)
                 .build().unwrap();

            kernel.gws(dims).arg_buf(&buffer);
        }

        // run kernel
        let res_dims = resultant_mat.m * resultant_mat.n;
        kernel.cmd().gwo(kernel.get_gwo())
                    .gws(res_dims)
                    .ewait_opt(None::<&EventList>)
                    .enew_opt(None::<&mut Event>)
                    .enq().unwrap();

        // Read results from the device into a vector (`::block` not shown):
        let mut res_vec = vec![0; res_dims];
        buffer.cmd()
            .queue(&queue)
            .offset(0)
            .read(&mut res_vec)
            .ewait_opt(None::<&EventList>)
            .enew_opt(None::<&mut Event>)
            .enq().unwrap();

        resultant_mat.mat = res_vec;
    }

}

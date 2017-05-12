extern crate ocl;
use ocl::ProQue;
use ocl::{flags, Platform, Device, Context, Queue, Program,
        Buffer, Kernel, Event, EventList};

use opencl::mem::CLBuffer;
use std::fmt;
use std::ptr;
use std::{mem, thread};

use matrix::{Matrix, /*Flag,*/ ReadOrder};
use std::sync::{Once, ONCE_INIT};

struct OpenCLHandler {
    platform : Platform,
    device : Device,
    context : Context,
}

// TODO: integrate error handling
//  test
impl OpenCLHandler {
    //
    //  Singleton impl of opencl objects for preprocessed access
    //
    pub fn new(&self) -> OpenCLHandler
    {
        // only call to self.opencl_components once
        unsafe {
            INIT.call_once(|| {
                self.platform = Platform::default();
                self.device = Device::first(self.platform);
                self.context = Context::builder()
                    .platform(self.platform)
                    .devices(self.device.clone())
                    .build().unwrap();
            });
        }
    }

    pub fn run(src: String, kernel_fn: String, kernel_args: &[Matrix], resultant_mat: &mut Matrix)
    {
        let program = Program::builder()
            .devices(self.device)
            .src(src)
            .build(&self.context).unwrap();

        let queue = Queue::new(&self.context, self.device, None).unwrap();
        let kernel = Kernel::new( kernel_fn, &program ).unwrap().queue(queue.clone())

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
        let mut res_vec = vec![0; res_dims]
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

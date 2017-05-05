rola
====

Rola (Rust Optimized Linear Algebra) is (will be) a Rust linear algebra
library aiming to be fast and robust with the intent of use in numerical
computing.

The scope of the project is to create fast, robust matrix and vector
operations, and to solve ``Ax = b`` as quickly and efficiently as possible.

Goals
-----

1. Idiomatic, clean Rust
2. Mathematically sound
3. Comparable speed to predecessors

Potentially look into GPU acceleration:
* [Vulkan (currently unstable)](https://github.com/tomaka/vulkano)
* OpenCL
  * https://github.com/luqmana/rust-opencl
  * https://github.com/cogciprocate/ocl

Research
--------

### References
* [LAPACK](https://github.com/reference-lapack/lapack) FORTRAN, tried and true
* [Blaze](https://bitbucket.org/blaze-lib/blaze) C++, cutting edge and [fast](https://bitbucket.org/blaze-lib/blaze/wiki/Benchmarks)
* [Eigen](https://bitbucket.org/eigen/eigen) Pure C++, clean and robust

### Study Material & Theory
* Numerical Analysis/Methods
  * [Numerical Computation of Matrix Functions (Smith)](http://www.maths.manchester.ac.uk/~higham/links/theses/smith02.pdf)
  * [Fundamental Numerical Methods and Data Analysis (Collins)](http://ads.harvard.edu/books/1990fnmd.book)
  * [Numerical Analysis (watermarked & difficult to read) (Ipsen)](http://www4.ncsu.edu/~ipsen/ps/OT113_Ipsen.pdf)
* Computation of Eigenvalues
  * [Olver (UMN)](http://www-users.math.umn.edu/~olver/num_/lnqr.pdf)
  * [gatech](https://www-old.math.gatech.edu/academic/courses/core/math2601/Web-notes/5num.pdf)
* Texts
  * [Numerical Analysis (Burden, Faires)](http://ins.sjtu.edu.cn/people/mtang/textbook.pdf)
  * [Numerical Analysis (Scott)](http://people.cs.uchicago.edu/~ridg/newna/nalrs.pdf)
  * [Lectures in Basic Computation Numerical Analysis (McDonough)](http://www.engr.uky.edu/~acfd/egr537-lctrs.pdf)
* arXiv.org
  * [Numerical Analysis Category](https://arxiv.org/list/math.NA/recent)

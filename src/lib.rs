/*!
# Cargo features

## CUDA version features

There are often multiple versions of CUDA installed, and different programs may
require different CUDA versions. Therefore it is important to ensure that the
FFI bindings are for the desired version of CUDA.

There are several features that pin the API bindings to specific CUDA versions:

- `cuda_6_5`, `cuda_7_0`, `cuda_7_5`, `cuda_8_0`, `cuda_9_0`, `cuda_9_1`,
  `cuda_9_2`, `cuda_10_0`, `cuda_10_1`

One and only one of these features must be set somewhere in the cargo dependency
graph in order for the cuda crate to be used, otherwise the crate will force a
compile-time error.

For further details on CUDA version compatibility, please see:
https://docs.nvidia.com/deploy/cuda-compatibility/index.html.

### Recommendations

It is recommended that _binary_ crates specify one and only one of these version
features, whereas _library_ crates should specify no features.

## Other Cargo features

- `cuda_sys`: This features substitutes the FFI bindings auto-generated by
  bindgen with the bindings provided by the cuda_sys
  [crate](https://crates.io/crates/cuda-sys). As of cuda_sys v0.2.0, these
  bindings are specific to CUDA 8.0 only.

- `fresh`: For maintainers of the crate itself. This feature generates fresh FFI
  bindings using bindgen from the build script.

*/

#[cfg(feature = "cuda_sys")]
extern crate cuda_sys;
#[macro_use] extern crate static_assertions;

pub mod blas;
pub mod driver;
pub mod extras;
#[cfg(not(feature = "cuda_sys"))]
pub mod ffi;
#[cfg(feature = "cuda_sys")]
pub mod ffi {
  pub use crate::ffi_via_cuda_sys::*;
}
#[cfg(feature = "cuda_sys")]
mod ffi_via_cuda_sys;
#[cfg(not(feature = "cuda_sys"))]
pub mod rand;
pub mod runtime;

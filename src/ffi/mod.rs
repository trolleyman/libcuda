#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub use self::v::cublas;
pub use self::v::cuda;
#[cfg(feature = "cuda_gte_8_0")]
pub use self::v::cuda_fp16;
pub use self::v::cuda_runtime_api;
pub use self::v::curand;
pub use self::v::driver_types;
#[cfg(feature = "cuda_gte_8_0")]
pub use self::v::library_types;

#[cfg(feature = "cuda_6_5")]
mod v {
  pub mod cublas            { use crate::ffi::driver_types::*;
                              include!("v6_5/_cublas.rs"); }
  pub mod cuda              { include!("v6_5/_cuda.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v6_5/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              include!("v6_5/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v6_5/_driver_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  6050);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        6050);
}

#[cfg(feature = "cuda_gte_7_0")]
mod v7_0 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  7000);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  7000);
}

#[cfg(feature = "cuda_7_0")]
mod v {
  pub mod cublas            { use crate::ffi::driver_types::*;
                              include!("v7_0/_cublas.rs"); }
  pub mod cuda              { include!("v7_0/_cuda.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v7_0/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              include!("v7_0/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v7_0/_driver_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  7000);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        7000);
}

#[cfg(feature = "cuda_gte_7_5")]
mod v7_5 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  7050);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  7050);
}

#[cfg(feature = "cuda_7_5")]
mod v {
  pub mod cublas            { use crate::ffi::driver_types::*;
                              include!("v7_5/_cublas.rs"); }
  pub mod cuda              { include!("v7_5/_cuda.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v7_5/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              include!("v7_5/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v7_5/_driver_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  7050);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        7050);
}

#[cfg(feature = "cuda_gte_8_0")]
mod v8_0 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  8000);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  8000);
}

#[cfg(feature = "cuda_8_0")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v8_0/_cublas.rs");
                              include!("v8_0/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v8_0/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v8_0/cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v8_0/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v8_0/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v8_0/_driver_types.rs"); }
  pub mod library_types     { include!("v8_0/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  8000);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        8000);
}

#[cfg(feature = "cuda_gte_9_0")]
mod v9_0 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  9000);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  9000);
}

#[cfg(feature = "cuda_9_0")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v9_0/_cublas.rs");
                              include!("v9_0/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v9_0/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v9_0/_cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v9_0/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v9_0/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v9_0/_driver_types.rs"); }
  pub mod library_types     { include!("v9_0/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  9000);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        9000);
}

#[cfg(feature = "cuda_gte_9_1")]
mod v9_1 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  9010);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  9010);
}

#[cfg(feature = "cuda_9_1")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v9_1/_cublas.rs");
                              include!("v9_1/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v9_1/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v9_1/_cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v9_1/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v9_1/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v9_1/_driver_types.rs"); }
  pub mod library_types     { include!("v9_1/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  9010);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        9010);
}

#[cfg(feature = "cuda_gte_9_2")]
mod v9_2 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >=  9020);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >=  9020);
}

#[cfg(feature = "cuda_9_2")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v9_2/_cublas.rs");
                              include!("v9_2/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v9_2/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v9_2/_cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v9_2/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v9_2/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v9_2/_driver_types.rs"); }
  pub mod library_types     { include!("v9_2/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION,  9020);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,        9020);
}

#[cfg(feature = "cuda_gte_10_0")]
mod v10_0 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >= 10000);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >= 10000);
}

#[cfg(feature = "cuda_10_0")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v10_0/_cublas.rs");
                              include!("v10_0/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v10_0/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v10_0/_cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v10_0/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v10_0/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v10_0/_driver_types.rs"); }
  pub mod library_types     { include!("v10_0/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION, 10000);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,       10000);
}

#[cfg(feature = "cuda_gte_10_1")]
mod v10_1 {
  const_assert!(cuda_api_version_gte; super::cuda::__CUDA_API_VERSION >= 10010);
  const_assert!(cuda_version_gte;     super::cuda::CUDA_VERSION       >= 10010);
}

#[cfg(feature = "cuda_10_1")]
mod v {
  pub mod cublas            { use crate::ffi::cuda_fp16::{__half};
                              use crate::ffi::driver_types::*;
                              pub use crate::ffi::library_types::{cudaDataType};
                              use crate::ffi::library_types::{libraryPropertyType};
                              include!("v10_1/_cublas.rs");
                              include!("v10_1/_cublas_cxx.rs"); }
  pub mod cuda              { include!("v10_1/_cuda.rs"); }
  pub mod cuda_fp16         { include!("v10_1/_cuda_fp16.rs"); }
  pub mod cuda_runtime_api  { use crate::ffi::driver_types::*;
                              include!("v10_1/_cuda_runtime_api.rs"); }
  pub mod curand            { use crate::ffi::driver_types::*;
                              use crate::ffi::library_types::*;
                              include!("v10_1/_curand.rs"); }
  pub mod driver_types      { use crate::ffi::cuda::*;
                              include!("v10_1/_driver_types.rs"); }
  pub mod library_types     { include!("v10_1/_library_types.rs"); }

  const_assert_eq!(cuda_api_version; self::cuda::__CUDA_API_VERSION, 10010);
  const_assert_eq!(cuda_version;     self::cuda::CUDA_VERSION,       10010);
}

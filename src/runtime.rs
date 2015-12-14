#![allow(missing_copy_implementations)]

use ffi::runtime::*;
use ffi::runtime::cudaError::{Success};

use libc::{c_void, c_int, c_uint, size_t};
use std::cell::{RefCell};
use std::mem::{transmute};
use std::ops::{Range};
use std::ptr::{null_mut};
use std::sync::{Arc};

#[repr(C)]
pub struct Dim3 {
  x: u32,
  y: u32,
  z: u32,
}

pub type CudaResult<T> = Result<T, CudaError>;

#[derive(Clone, Copy, Debug)]
pub struct CudaError(cudaError);

impl CudaError {
  /*pub fn get_name(&self) -> &mut str {
    let &CudaError(e) = self;
    unsafe {
      from_raw_mut_buf(cudaGetErrorName(e));
    }
  }

  pub fn get_string(&self) -> &mut str {
  }*/

  pub fn get_code(&self) -> i64 {
    let &CudaError(ref e) = self;
    unsafe {
      transmute(e)
    }
  }
}

pub fn cuda_get_driver_version() -> CudaResult<i32> {
  unsafe {
    let mut version: c_int = 0;
    match cudaDriverGetVersion(&mut version as *mut c_int) {
      Success => Ok(version as i32),
      e => Err(CudaError(e)),
    }
  }
}

pub fn cuda_get_runtime_version() -> CudaResult<i32> {
  unsafe {
    let mut version: c_int = 0;
    match cudaRuntimeGetVersion(&mut version as *mut c_int) {
      Success => Ok(version as i32),
      e => Err(CudaError(e)),
    }
  }
}

// TODO: device flags.

pub struct CudaDevice;

impl CudaDevice {
  pub fn count() -> CudaResult<usize> {
    let mut count: c_int = 0;
    unsafe {
      match cudaGetDeviceCount(&mut count as *mut c_int) {
        Success => Ok(count as usize),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn iter() -> CudaResult<Range<usize>> {
    Self::count().and_then(|count| Ok((0 .. count)))
  }

  pub fn get_properties(&self) {
    /*unsafe {
      match cudaGetProperties(...) {
      }
    }*/
  }

  pub fn get_current() -> CudaResult<usize> {
    let mut index: c_int = 0;
    match unsafe { cudaGetDevice(&mut index as *mut c_int) } {
      Success => Ok(index as usize),
      e => Err(CudaError(e)),
    }
  }

  pub fn set_current(index: usize) -> CudaResult<()> {
    unsafe {
      match cudaSetDevice(index as c_int) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn reset() -> CudaResult<()> {
    match unsafe { cudaDeviceReset() } {
      Success => Ok(()),
      e => Err(CudaError(e)),
    }
  }

  pub fn set_flags(flags: u32) -> CudaResult<()> {
    match unsafe { cudaSetDeviceFlags(flags as c_uint) } {
      Success => Ok(()),
      e => Err(CudaError(e)),
    }
  }

  pub fn get_attribute(device_idx: usize, ffi_attr: cudaDeviceAttr) -> CudaResult<i32> {
    let mut value: c_int = 0;
    match unsafe { cudaDeviceGetAttribute(&mut value as *mut c_int, ffi_attr, device_idx as c_int) } {
      Success => Ok(value as i32),
      e => Err(CudaError(e)),
    }
  }

  pub fn can_access_peer(idx: usize, peer_idx: usize) -> CudaResult<bool> {
    unsafe {
      let mut access: c_int = 0;
      match cudaDeviceCanAccessPeer(&mut access as *mut c_int, idx as c_int, peer_idx as c_int) {
        Success => Ok(access != 0),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn enable_peer_access(peer_idx: usize) -> CudaResult<()> {
    unsafe {
      match cudaDeviceEnablePeerAccess(peer_idx as c_int, 0) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn disable_peer_access(peer_idx: usize) -> CudaResult<()> {
    unsafe {
      match cudaDeviceDisablePeerAccess(peer_idx as c_int) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }
}

pub struct CudaStream {
  pub ptr: cudaStream_t,
}

impl !Send for CudaStream {
}

impl !Sync for CudaStream {
}

impl Drop for CudaStream {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      unsafe {
        match cudaStreamDestroy(self.ptr) {
          Success => (),
          e => panic!("FATAL: CudaStream::drop() failed: {}", CudaError(e).get_code()),
        }
      }
    }
  }
}

impl CudaStream {
  pub fn default() -> CudaStream {
    CudaStream{
      ptr: null_mut(),
    }
  }

  pub fn create() -> CudaResult<CudaStream> {
    unsafe {
      let mut ptr: cudaStream_t = null_mut();
      match cudaStreamCreate(&mut ptr as *mut cudaStream_t) {
        Success => {
          Ok(CudaStream{
            ptr: ptr,
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn create_with_flags(flags: i32) -> CudaResult<CudaStream> {
    unsafe {
      // TODO: flags.
      let mut ptr: cudaStream_t = null_mut();
      match cudaStreamCreate(&mut ptr as *mut cudaStream_t) {
        Success => {
          Ok(CudaStream{
            ptr: ptr,
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn create_with_priority(flags: i32, priority: i32) -> CudaResult<CudaStream> {
    unsafe {
      // TODO: flags and priority.
      let mut ptr: cudaStream_t = null_mut();
      match cudaStreamCreate(&mut ptr as *mut cudaStream_t) {
        Success => {
          Ok(CudaStream{
            ptr: ptr,
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn synchronize(&self) -> CudaResult<()> {
    unsafe {
      match cudaStreamSynchronize(self.ptr) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn wait_event(&self, event: &CudaEvent) -> CudaResult<()> {
    match unsafe { cudaStreamWaitEvent(self.ptr, event.ptr, 0) } {
      Success => Ok(()),
      e => Err(CudaError(e))
    }
  }

  pub fn wait_shared_event(&self, event: &SharedCudaEvent) -> CudaResult<()> {
    match unsafe { cudaStreamWaitEvent(self.ptr, event.inner.ptr, 0) } {
      Success => Ok(()),
      e => Err(CudaError(e))
    }
  }
}

pub enum CudaEventStatus {
  Complete,
  NotReady,
}

pub struct CudaEvent {
  pub ptr: cudaEvent_t,
}

// FIXME(20151211): events may be used but not freed outside their context
// (e.g., by cudaStreamWaitEvent).

impl !Send for CudaEvent {}
impl !Sync for CudaEvent {}

//unsafe impl Send for CudaEvent {}
//unsafe impl Sync for CudaEvent {}

impl Drop for CudaEvent {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      unsafe {
        match cudaEventDestroy(self.ptr) {
          Success => (),
          e => panic!("FATAL: CudaEvent::drop(): failed to destroy: {:?}", e),
        }
      }
    }
  }
}

impl CudaEvent {
  pub fn create() -> CudaResult<CudaEvent> {
    unsafe {
      let mut ptr = 0 as cudaEvent_t;
      match cudaEventCreate(&mut ptr as *mut cudaEvent_t) {
        Success => {
          Ok(CudaEvent{
            ptr: ptr,
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn create_fastest() -> CudaResult<CudaEvent> {
    Self::create_with_flags(0x02)
  }

  pub fn create_with_flags(flags: u32) -> CudaResult<CudaEvent> {
    unsafe {
      let mut ptr = 0 as cudaEvent_t;
      match cudaEventCreateWithFlags(&mut ptr as *mut cudaEvent_t, flags as c_uint) {
        Success => {
          Ok(CudaEvent{
            ptr: ptr,
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn record(&self, stream: &CudaStream) -> CudaResult<()> {
    unsafe {
      match cudaEventRecord(self.ptr, stream.ptr) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn query(&self) -> CudaResult<CudaEventStatus> {
    match unsafe { cudaEventQuery(self.ptr) } {
      Success => Ok(CudaEventStatus::Complete),
      e => match e {
        cudaError::NotReady => Ok(CudaEventStatus::NotReady),
        e => Err(CudaError(e)),
      },
    }
  }

  pub fn synchronize(&self) -> CudaResult<()> {
    unsafe {
      match cudaEventSynchronize(self.ptr) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }
}

thread_local!(static EVENT_POOL: RefCell<EventReclaimPool> = RefCell::new(EventReclaimPool::new()));

struct EventReclaimPool {
  events: Vec<Arc<RawCudaEvent>>,
}

impl EventReclaimPool {
  pub fn new() -> EventReclaimPool {
    EventReclaimPool{
      events: vec![],
    }
  }

  pub fn add_event(&mut self, event: Arc<RawCudaEvent>) {
    self.events.push(event);
  }

  pub fn collect_garbage(&mut self) {
    let mut i = 0;
    while i < self.events.len() {
      if Arc::strong_count(&self.events[i]) == 1 {
        let event = self.events.swap_remove(i);
        event.unsafe_drop();
      } else {
        i += 1;
      }
    }
  }
}

struct RawCudaEvent {
  ptr:  cudaEvent_t,
}

impl RawCudaEvent {
  /// RawCudaEvent must be manually dropped, otherwise Drop::drop() tends to
  /// result in a CudartUnloading error.
  fn unsafe_drop(&self) {
    if !self.ptr.is_null() {
      unsafe {
        match cudaEventDestroy(self.ptr) {
          Success => (),
          e => panic!("FATAL: RawCudaEvent::drop(): failed to destroy: {:?}", e),
        }
      }
    }
  }
}

pub struct OwnedCudaEvent {
  inner:  Arc<RawCudaEvent>,
}

impl !Send for OwnedCudaEvent {}
impl !Sync for OwnedCudaEvent {}

impl Drop for OwnedCudaEvent {
  fn drop(&mut self) {
    EVENT_POOL.with(|pool| {
      let mut pool = pool.borrow_mut();
      pool.collect_garbage();
      pool.add_event(self.inner.clone());
    });
  }
}

impl OwnedCudaEvent {
  pub fn create_fastest() -> CudaResult<OwnedCudaEvent> {
    Self::create_with_flags(0x02)
  }

  pub fn create_with_flags(flags: u32) -> CudaResult<OwnedCudaEvent> {
    EVENT_POOL.with(|pool| {
      let mut pool = pool.borrow_mut();
      pool.collect_garbage();
    });
    unsafe {
      let mut ptr = 0 as cudaEvent_t;
      match cudaEventCreateWithFlags(&mut ptr as *mut cudaEvent_t, flags as c_uint) {
        Success => {
          Ok(OwnedCudaEvent{
            inner:  Arc::new(RawCudaEvent{ptr: ptr}),
          })
        },
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn share(&self) -> SharedCudaEvent {
    SharedCudaEvent{inner: self.inner.clone()}
  }

  pub fn record(&self, stream: &CudaStream) -> CudaResult<()> {
    unsafe {
      match cudaEventRecord(self.inner.ptr, stream.ptr) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }

  pub fn query(&self) -> CudaResult<CudaEventStatus> {
    match unsafe { cudaEventQuery(self.inner.ptr) } {
      Success => Ok(CudaEventStatus::Complete),
      e => match e {
        cudaError::NotReady => Ok(CudaEventStatus::NotReady),
        e => Err(CudaError(e)),
      },
    }
  }

  pub fn synchronize(&self) -> CudaResult<()> {
    unsafe {
      match cudaEventSynchronize(self.inner.ptr) {
        Success => Ok(()),
        e => Err(CudaError(e)),
      }
    }
  }
}

pub struct SharedCudaEvent {
  inner:  Arc<RawCudaEvent>,
}

unsafe impl Send for SharedCudaEvent {}
unsafe impl Sync for SharedCudaEvent {}

impl Clone for SharedCudaEvent {
  fn clone(&self) -> SharedCudaEvent {
    SharedCudaEvent{inner: self.inner.clone()}
  }
}

impl SharedCudaEvent {
}

#[derive(Clone, Copy, Debug)]
pub struct CudaMemInfo {
  pub used: usize,
  pub free: usize,
  pub total: usize,
}

pub fn cuda_get_mem_info() -> CudaResult<CudaMemInfo> {
  unsafe {
    let mut free: size_t = 0;
    let mut total: size_t = 0;
    match cudaMemGetInfo(&mut free as *mut size_t, &mut total as *mut size_t) {
      Success => Ok(CudaMemInfo{
        used: (total - free) as usize,
        free: free as usize,
        total: total as usize,
      }),
      e => Err(CudaError(e)),
    }
  }
}

pub unsafe fn cuda_alloc_pinned(size: usize, flags: u32) -> CudaResult<*mut u8> {
  let mut ptr = 0 as *mut c_void;
  match cudaHostAlloc(&mut ptr as *mut *mut c_void, size as size_t, flags) {
    Success => Ok(ptr as *mut u8),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_free_pinned(ptr: *mut u8) -> CudaResult<()> {
  match cudaFreeHost(ptr as *mut c_void) {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_alloc_device(size: usize) -> CudaResult<*mut u8> {
  let mut ptr = 0 as *mut c_void;
  match cudaMalloc(&mut ptr as *mut *mut c_void, size) {
    Success => Ok(ptr as *mut u8),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_free_device(dev_ptr: *mut u8) -> CudaResult<()> {
  match cudaFree(dev_ptr as *mut c_void) {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memset(dev_ptr: *mut u8, value: i32, size: usize) -> CudaResult<()> {
  match cudaMemset(dev_ptr as *mut c_void, value, size as size_t) {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memset_async(dev_ptr: *mut u8, value: i32, size: usize, stream: &CudaStream) -> CudaResult<()> {
  match cudaMemsetAsync(dev_ptr as *mut c_void, value, size as size_t, stream.ptr) {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub enum CudaMemcpyKind {
  HostToHost,
  HostToDevice,
  DeviceToHost,
  DeviceToDevice,
  Unified,
}

pub unsafe fn cuda_memcpy(
    dst: *mut u8,
    src: *const u8,
    size: usize,
    kind: CudaMemcpyKind) -> CudaResult<()>
{
  let kind = match kind {
    CudaMemcpyKind::HostToHost      => cudaMemcpyKind::HostToHost,
    CudaMemcpyKind::HostToDevice    => cudaMemcpyKind::HostToDevice,
    CudaMemcpyKind::DeviceToHost    => cudaMemcpyKind::DeviceToHost,
    CudaMemcpyKind::DeviceToDevice  => cudaMemcpyKind::DeviceToDevice,
    CudaMemcpyKind::Unified         => cudaMemcpyKind::Default,
  };
  match cudaMemcpy(
      dst as *mut c_void,
      src as *const c_void,
      size as size_t,
      kind)
  {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memcpy_async(
    dst: *mut u8,
    src: *const u8,
    size: usize,
    kind: CudaMemcpyKind,
    stream: &CudaStream) -> CudaResult<()>
{
  let kind = match kind {
    CudaMemcpyKind::HostToHost      => cudaMemcpyKind::HostToHost,
    CudaMemcpyKind::HostToDevice    => cudaMemcpyKind::HostToDevice,
    CudaMemcpyKind::DeviceToHost    => cudaMemcpyKind::DeviceToHost,
    CudaMemcpyKind::DeviceToDevice  => cudaMemcpyKind::DeviceToDevice,
    CudaMemcpyKind::Unified         => cudaMemcpyKind::Default,
  };
  match cudaMemcpyAsync(
      dst as *mut c_void,
      src as *const c_void,
      size as size_t,
      kind,
      stream.ptr)
  {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memcpy_peer_async(
    dst: *mut u8, dst_device_idx: usize,
    src: *const u8, src_device_idx: usize,
    size: usize,
    stream: &CudaStream) -> CudaResult<()>
{
  match cudaMemcpyPeerAsync(
      dst as *mut c_void, dst_device_idx as c_int,
      src as *const c_void, src_device_idx as c_int,
      size as size_t,
      stream.ptr)
  {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memcpy_2d(
    dst: *mut u8, dst_pitch: usize,
    src: *const u8, src_pitch: usize,
    width: usize, height: usize,
    kind: CudaMemcpyKind) -> CudaResult<()>
{
  let kind = match kind {
    CudaMemcpyKind::HostToHost      => cudaMemcpyKind::HostToHost,
    CudaMemcpyKind::HostToDevice    => cudaMemcpyKind::HostToDevice,
    CudaMemcpyKind::DeviceToHost    => cudaMemcpyKind::DeviceToHost,
    CudaMemcpyKind::DeviceToDevice  => cudaMemcpyKind::DeviceToDevice,
    CudaMemcpyKind::Unified         => cudaMemcpyKind::Default,
  };
  match cudaMemcpy2D(
      dst as *mut c_void, dst_pitch as size_t,
      src as *const c_void, src_pitch as size_t,
      width as size_t, height as size_t,
      kind)
  {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

pub unsafe fn cuda_memcpy_2d_async(
    dst: *mut u8, dst_pitch: usize,
    src: *const u8, src_pitch: usize,
    width: usize, height: usize,
    kind: CudaMemcpyKind,
    stream: &CudaStream) -> CudaResult<()>
{
  let kind = match kind {
    CudaMemcpyKind::HostToHost      => cudaMemcpyKind::HostToHost,
    CudaMemcpyKind::HostToDevice    => cudaMemcpyKind::HostToDevice,
    CudaMemcpyKind::DeviceToHost    => cudaMemcpyKind::DeviceToHost,
    CudaMemcpyKind::DeviceToDevice  => cudaMemcpyKind::DeviceToDevice,
    CudaMemcpyKind::Unified         => cudaMemcpyKind::Default,
  };
  match cudaMemcpy2DAsync(
      dst as *mut c_void, dst_pitch as size_t,
      src as *const c_void, src_pitch as size_t,
      width as size_t, height as size_t,
      kind,
      stream.ptr)
  {
    Success => Ok(()),
    e => Err(CudaError(e)),
  }
}

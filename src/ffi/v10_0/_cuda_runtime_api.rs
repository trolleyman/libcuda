/* automatically generated by rust-bindgen */

extern "C" {
    pub fn cudaDeviceReset() -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSynchronize() -> cudaError_t;
}
extern "C" {
    pub fn cudaGetErrorString(error: cudaError_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cudaGetDeviceCount(count: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDeviceProperties(
        prop: *mut cudaDeviceProp,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetAttribute(
        value: *mut ::std::os::raw::c_int,
        attr: cudaDeviceAttr,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDevice(device: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDevice(device: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDeviceFlags(flags: ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDeviceFlags(flags: *mut ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreateWithFlags(
        pStream: *mut cudaStream_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreateWithPriority(
        pStream: *mut cudaStream_t,
        flags: ::std::os::raw::c_uint,
        priority: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamWaitEvent(
        stream: cudaStream_t,
        event: cudaEvent_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
pub type cudaStreamCallback_t = ::std::option::Option<
    unsafe extern "C" fn(
        stream: cudaStream_t,
        status: cudaError_t,
        userData: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn cudaStreamAddCallback(
        stream: cudaStream_t,
        callback: cudaStreamCallback_t,
        userData: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamAttachMemAsync(
        stream: cudaStream_t,
        devPtr: *mut ::std::os::raw::c_void,
        length: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreateWithFlags(
        event: *mut cudaEvent_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventElapsedTime(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocManaged(
        devPtr: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMalloc(devPtr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocHost(ptr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaFree(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeHost(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostAlloc(
        pHost: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostRegister(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostUnregister(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostGetDevicePointer(
        pDevice: *mut *mut ::std::os::raw::c_void,
        pHost: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostGetFlags(
        pFlags: *mut ::std::os::raw::c_uint,
        pHost: *mut ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyPeer(
        dst: *mut ::std::os::raw::c_void,
        dstDevice: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        srcDevice: ::std::os::raw::c_int,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2D(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyAsync(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyPeerAsync(
        dst: *mut ::std::os::raw::c_void,
        dstDevice: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        srcDevice: ::std::os::raw::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DAsync(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset(
        devPtr: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemsetAsync(
        devPtr: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPrefetchAsync(
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
        dstDevice: ::std::os::raw::c_int,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemAdvise(
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemRangeGetAttribute(
        data: *mut ::std::os::raw::c_void,
        dataSize: usize,
        attribute: cudaMemRangeAttribute,
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemRangeGetAttributes(
        data: *mut *mut ::std::os::raw::c_void,
        dataSizes: *mut usize,
        attributes: *mut cudaMemRangeAttribute,
        numAttributes: usize,
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut ::std::os::raw::c_int,
        device: ::std::os::raw::c_int,
        peerDevice: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceEnablePeerAccess(
        peerDevice: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceDisablePeerAccess(peerDevice: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsResourceSetMapFlags(
        resource: cudaGraphicsResource_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsMapResources(
        count: ::std::os::raw::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsUnmapResources(
        count: ::std::os::raw::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut ::std::os::raw::c_void,
        size: *mut usize,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDriverGetVersion(driverVersion: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaRuntimeGetVersion(runtimeVersion: *mut ::std::os::raw::c_int) -> cudaError_t;
}

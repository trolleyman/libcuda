/* automatically generated by rust-bindgen */

pub const __CUDA_API_VERSION: u32 = 6050;
pub const CUDA_VERSION: u32 = 6050;
pub type CUdeviceptr = ::std::os::raw::c_ulonglong;
pub type CUdevice = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUctx_st {
    _unused: [u8; 0],
}
pub type CUcontext = *mut CUctx_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmod_st {
    _unused: [u8; 0],
}
pub type CUmodule = *mut CUmod_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUfunc_st {
    _unused: [u8; 0],
}
pub type CUfunction = *mut CUfunc_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUarray_st {
    _unused: [u8; 0],
}
pub type CUarray = *mut CUarray_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUtexref_st {
    _unused: [u8; 0],
}
pub type CUtexref = *mut CUtexref_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUsurfref_st {
    _unused: [u8; 0],
}
pub type CUsurfref = *mut CUsurfref_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
pub type CUevent = *mut CUevent_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type CUstream = *mut CUstream_st;
#[repr(C)]
pub struct CUuuid_st {
    pub bytes: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout_CUuuid_st() {
    assert_eq!(
        ::std::mem::size_of::<CUuuid_st>(),
        16usize,
        concat!("Size of: ", stringify!(CUuuid_st))
    );
    assert_eq!(
        ::std::mem::align_of::<CUuuid_st>(),
        1usize,
        concat!("Alignment of ", stringify!(CUuuid_st))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CUuuid_st>())).bytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CUuuid_st),
            "::",
            stringify!(bytes)
        )
    );
}
pub type CUuuid = CUuuid_st;
pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK: CUdevice_attribute_enum = 1;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X: CUdevice_attribute_enum = 2;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y: CUdevice_attribute_enum = 3;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z: CUdevice_attribute_enum = 4;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X: CUdevice_attribute_enum = 5;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y: CUdevice_attribute_enum = 6;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z: CUdevice_attribute_enum = 7;
pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = 8;
pub const CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = 8;
pub const CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY: CUdevice_attribute_enum = 9;
pub const CU_DEVICE_ATTRIBUTE_WARP_SIZE: CUdevice_attribute_enum = 10;
pub const CU_DEVICE_ATTRIBUTE_MAX_PITCH: CUdevice_attribute_enum = 11;
pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = 12;
pub const CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = 12;
pub const CU_DEVICE_ATTRIBUTE_CLOCK_RATE: CUdevice_attribute_enum = 13;
pub const CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT: CUdevice_attribute_enum = 14;
pub const CU_DEVICE_ATTRIBUTE_GPU_OVERLAP: CUdevice_attribute_enum = 15;
pub const CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT: CUdevice_attribute_enum = 16;
pub const CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT: CUdevice_attribute_enum = 17;
pub const CU_DEVICE_ATTRIBUTE_INTEGRATED: CUdevice_attribute_enum = 18;
pub const CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY: CUdevice_attribute_enum = 19;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_MODE: CUdevice_attribute_enum = 20;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH: CUdevice_attribute_enum = 21;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH: CUdevice_attribute_enum = 22;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT: CUdevice_attribute_enum = 23;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH: CUdevice_attribute_enum = 24;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT: CUdevice_attribute_enum = 25;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH: CUdevice_attribute_enum = 26;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH: CUdevice_attribute_enum = 27;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = 28;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS: CUdevice_attribute_enum = 29;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH: CUdevice_attribute_enum = 27;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT: CUdevice_attribute_enum = 28;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES: CUdevice_attribute_enum = 29;
pub const CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT: CUdevice_attribute_enum = 30;
pub const CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS: CUdevice_attribute_enum = 31;
pub const CU_DEVICE_ATTRIBUTE_ECC_ENABLED: CUdevice_attribute_enum = 32;
pub const CU_DEVICE_ATTRIBUTE_PCI_BUS_ID: CUdevice_attribute_enum = 33;
pub const CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID: CUdevice_attribute_enum = 34;
pub const CU_DEVICE_ATTRIBUTE_TCC_DRIVER: CUdevice_attribute_enum = 35;
pub const CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE: CUdevice_attribute_enum = 36;
pub const CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH: CUdevice_attribute_enum = 37;
pub const CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE: CUdevice_attribute_enum = 38;
pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 39;
pub const CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT: CUdevice_attribute_enum = 40;
pub const CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING: CUdevice_attribute_enum = 41;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH: CUdevice_attribute_enum = 42;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS: CUdevice_attribute_enum = 43;
pub const CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER: CUdevice_attribute_enum = 44;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH: CUdevice_attribute_enum = 45;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT: CUdevice_attribute_enum = 46;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE: CUdevice_attribute_enum = 47;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE: CUdevice_attribute_enum = 48;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE: CUdevice_attribute_enum = 49;
pub const CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID: CUdevice_attribute_enum = 50;
pub const CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT: CUdevice_attribute_enum = 51;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH: CUdevice_attribute_enum = 52;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = 53;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = 54;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH: CUdevice_attribute_enum = 55;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH: CUdevice_attribute_enum = 56;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT: CUdevice_attribute_enum = 57;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH: CUdevice_attribute_enum = 58;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT: CUdevice_attribute_enum = 59;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH: CUdevice_attribute_enum = 60;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH: CUdevice_attribute_enum = 61;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS: CUdevice_attribute_enum = 62;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH: CUdevice_attribute_enum = 63;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = 64;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS: CUdevice_attribute_enum = 65;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH: CUdevice_attribute_enum = 66;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = 67;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = 68;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH: CUdevice_attribute_enum = 69;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH: CUdevice_attribute_enum = 70;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT: CUdevice_attribute_enum = 71;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH: CUdevice_attribute_enum = 72;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = 73;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT: CUdevice_attribute_enum = 74;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: CUdevice_attribute_enum = 75;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: CUdevice_attribute_enum = 76;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = 77;
pub const CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED: CUdevice_attribute_enum = 78;
pub const CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = 79;
pub const CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = 80;
pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 81;
pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 82;
pub const CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY: CUdevice_attribute_enum = 83;
pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD: CUdevice_attribute_enum = 84;
pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID: CUdevice_attribute_enum = 85;
pub const CU_DEVICE_ATTRIBUTE_MAX: CUdevice_attribute_enum = 86;
pub type CUdevice_attribute_enum = u32;
pub use self::CUdevice_attribute_enum as CUdevice_attribute;
pub const CU_POINTER_ATTRIBUTE_CONTEXT: CUpointer_attribute_enum = 1;
pub const CU_POINTER_ATTRIBUTE_MEMORY_TYPE: CUpointer_attribute_enum = 2;
pub const CU_POINTER_ATTRIBUTE_DEVICE_POINTER: CUpointer_attribute_enum = 3;
pub const CU_POINTER_ATTRIBUTE_HOST_POINTER: CUpointer_attribute_enum = 4;
pub const CU_POINTER_ATTRIBUTE_P2P_TOKENS: CUpointer_attribute_enum = 5;
pub const CU_POINTER_ATTRIBUTE_SYNC_MEMOPS: CUpointer_attribute_enum = 6;
pub const CU_POINTER_ATTRIBUTE_BUFFER_ID: CUpointer_attribute_enum = 7;
pub const CU_POINTER_ATTRIBUTE_IS_MANAGED: CUpointer_attribute_enum = 8;
pub type CUpointer_attribute_enum = u32;
pub use self::CUpointer_attribute_enum as CUpointer_attribute;
pub const CU_JIT_MAX_REGISTERS: CUjit_option_enum = 0;
pub const CU_JIT_THREADS_PER_BLOCK: CUjit_option_enum = 1;
pub const CU_JIT_WALL_TIME: CUjit_option_enum = 2;
pub const CU_JIT_INFO_LOG_BUFFER: CUjit_option_enum = 3;
pub const CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = 4;
pub const CU_JIT_ERROR_LOG_BUFFER: CUjit_option_enum = 5;
pub const CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = 6;
pub const CU_JIT_OPTIMIZATION_LEVEL: CUjit_option_enum = 7;
pub const CU_JIT_TARGET_FROM_CUCONTEXT: CUjit_option_enum = 8;
pub const CU_JIT_TARGET: CUjit_option_enum = 9;
pub const CU_JIT_FALLBACK_STRATEGY: CUjit_option_enum = 10;
pub const CU_JIT_GENERATE_DEBUG_INFO: CUjit_option_enum = 11;
pub const CU_JIT_LOG_VERBOSE: CUjit_option_enum = 12;
pub const CU_JIT_GENERATE_LINE_INFO: CUjit_option_enum = 13;
pub const CU_JIT_CACHE_MODE: CUjit_option_enum = 14;
pub const CU_JIT_NUM_OPTIONS: CUjit_option_enum = 15;
pub type CUjit_option_enum = u32;
pub use self::CUjit_option_enum as CUjit_option;
pub const CU_JIT_INPUT_CUBIN: CUjitInputType_enum = 0;
pub const CU_JIT_INPUT_PTX: CUjitInputType_enum = 1;
pub const CU_JIT_INPUT_FATBINARY: CUjitInputType_enum = 2;
pub const CU_JIT_INPUT_OBJECT: CUjitInputType_enum = 3;
pub const CU_JIT_INPUT_LIBRARY: CUjitInputType_enum = 4;
pub const CU_JIT_NUM_INPUT_TYPES: CUjitInputType_enum = 5;
pub type CUjitInputType_enum = u32;
pub use self::CUjitInputType_enum as CUjitInputType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlinkState_st {
    _unused: [u8; 0],
}
pub type CUlinkState = *mut CUlinkState_st;
pub const CUDA_SUCCESS: cudaError_enum = 0;
pub const CUDA_ERROR_INVALID_VALUE: cudaError_enum = 1;
pub const CUDA_ERROR_OUT_OF_MEMORY: cudaError_enum = 2;
pub const CUDA_ERROR_NOT_INITIALIZED: cudaError_enum = 3;
pub const CUDA_ERROR_DEINITIALIZED: cudaError_enum = 4;
pub const CUDA_ERROR_PROFILER_DISABLED: cudaError_enum = 5;
pub const CUDA_ERROR_PROFILER_NOT_INITIALIZED: cudaError_enum = 6;
pub const CUDA_ERROR_PROFILER_ALREADY_STARTED: cudaError_enum = 7;
pub const CUDA_ERROR_PROFILER_ALREADY_STOPPED: cudaError_enum = 8;
pub const CUDA_ERROR_NO_DEVICE: cudaError_enum = 100;
pub const CUDA_ERROR_INVALID_DEVICE: cudaError_enum = 101;
pub const CUDA_ERROR_INVALID_IMAGE: cudaError_enum = 200;
pub const CUDA_ERROR_INVALID_CONTEXT: cudaError_enum = 201;
pub const CUDA_ERROR_CONTEXT_ALREADY_CURRENT: cudaError_enum = 202;
pub const CUDA_ERROR_MAP_FAILED: cudaError_enum = 205;
pub const CUDA_ERROR_UNMAP_FAILED: cudaError_enum = 206;
pub const CUDA_ERROR_ARRAY_IS_MAPPED: cudaError_enum = 207;
pub const CUDA_ERROR_ALREADY_MAPPED: cudaError_enum = 208;
pub const CUDA_ERROR_NO_BINARY_FOR_GPU: cudaError_enum = 209;
pub const CUDA_ERROR_ALREADY_ACQUIRED: cudaError_enum = 210;
pub const CUDA_ERROR_NOT_MAPPED: cudaError_enum = 211;
pub const CUDA_ERROR_NOT_MAPPED_AS_ARRAY: cudaError_enum = 212;
pub const CUDA_ERROR_NOT_MAPPED_AS_POINTER: cudaError_enum = 213;
pub const CUDA_ERROR_ECC_UNCORRECTABLE: cudaError_enum = 214;
pub const CUDA_ERROR_UNSUPPORTED_LIMIT: cudaError_enum = 215;
pub const CUDA_ERROR_CONTEXT_ALREADY_IN_USE: cudaError_enum = 216;
pub const CUDA_ERROR_PEER_ACCESS_UNSUPPORTED: cudaError_enum = 217;
pub const CUDA_ERROR_INVALID_PTX: cudaError_enum = 218;
pub const CUDA_ERROR_INVALID_GRAPHICS_CONTEXT: cudaError_enum = 219;
pub const CUDA_ERROR_INVALID_SOURCE: cudaError_enum = 300;
pub const CUDA_ERROR_FILE_NOT_FOUND: cudaError_enum = 301;
pub const CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND: cudaError_enum = 302;
pub const CUDA_ERROR_SHARED_OBJECT_INIT_FAILED: cudaError_enum = 303;
pub const CUDA_ERROR_OPERATING_SYSTEM: cudaError_enum = 304;
pub const CUDA_ERROR_INVALID_HANDLE: cudaError_enum = 400;
pub const CUDA_ERROR_NOT_FOUND: cudaError_enum = 500;
pub const CUDA_ERROR_NOT_READY: cudaError_enum = 600;
pub const CUDA_ERROR_ILLEGAL_ADDRESS: cudaError_enum = 700;
pub const CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES: cudaError_enum = 701;
pub const CUDA_ERROR_LAUNCH_TIMEOUT: cudaError_enum = 702;
pub const CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING: cudaError_enum = 703;
pub const CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED: cudaError_enum = 704;
pub const CUDA_ERROR_PEER_ACCESS_NOT_ENABLED: cudaError_enum = 705;
pub const CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE: cudaError_enum = 708;
pub const CUDA_ERROR_CONTEXT_IS_DESTROYED: cudaError_enum = 709;
pub const CUDA_ERROR_ASSERT: cudaError_enum = 710;
pub const CUDA_ERROR_TOO_MANY_PEERS: cudaError_enum = 711;
pub const CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED: cudaError_enum = 712;
pub const CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED: cudaError_enum = 713;
pub const CUDA_ERROR_HARDWARE_STACK_ERROR: cudaError_enum = 714;
pub const CUDA_ERROR_ILLEGAL_INSTRUCTION: cudaError_enum = 715;
pub const CUDA_ERROR_MISALIGNED_ADDRESS: cudaError_enum = 716;
pub const CUDA_ERROR_INVALID_ADDRESS_SPACE: cudaError_enum = 717;
pub const CUDA_ERROR_INVALID_PC: cudaError_enum = 718;
pub const CUDA_ERROR_LAUNCH_FAILED: cudaError_enum = 719;
pub const CUDA_ERROR_NOT_PERMITTED: cudaError_enum = 800;
pub const CUDA_ERROR_NOT_SUPPORTED: cudaError_enum = 801;
pub const CUDA_ERROR_UNKNOWN: cudaError_enum = 999;
pub type cudaError_enum = u32;
pub use self::cudaError_enum as CUresult;
pub type CUstreamCallback = ::std::option::Option<
    unsafe extern "C" fn(
        hStream: CUstream,
        status: CUresult,
        userData: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn cuGetErrorString(error: CUresult, pStr: *mut *const ::std::os::raw::c_char) -> CUresult;
}
extern "C" {
    pub fn cuGetErrorName(error: CUresult, pStr: *mut *const ::std::os::raw::c_char) -> CUresult;
}
extern "C" {
    pub fn cuInit(Flags: ::std::os::raw::c_uint) -> CUresult;
}
extern "C" {
    pub fn cuDriverGetVersion(driverVersion: *mut ::std::os::raw::c_int) -> CUresult;
}
extern "C" {
    pub fn cuDeviceGet(device: *mut CUdevice, ordinal: ::std::os::raw::c_int) -> CUresult;
}
extern "C" {
    pub fn cuDeviceGetCount(count: *mut ::std::os::raw::c_int) -> CUresult;
}
extern "C" {
    pub fn cuDeviceGetName(
        name: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        dev: CUdevice,
    ) -> CUresult;
}
extern "C" {
    pub fn cuDeviceGetAttribute(
        pi: *mut ::std::os::raw::c_int,
        attrib: CUdevice_attribute,
        dev: CUdevice,
    ) -> CUresult;
}
extern "C" {
    pub fn cuCtxCreate_v2(
        pctx: *mut CUcontext,
        flags: ::std::os::raw::c_uint,
        dev: CUdevice,
    ) -> CUresult;
}
extern "C" {
    pub fn cuCtxDestroy_v2(ctx: CUcontext) -> CUresult;
}
extern "C" {
    pub fn cuCtxPushCurrent_v2(ctx: CUcontext) -> CUresult;
}
extern "C" {
    pub fn cuCtxPopCurrent_v2(pctx: *mut CUcontext) -> CUresult;
}
extern "C" {
    pub fn cuCtxSetCurrent(ctx: CUcontext) -> CUresult;
}
extern "C" {
    pub fn cuCtxGetCurrent(pctx: *mut CUcontext) -> CUresult;
}
extern "C" {
    pub fn cuCtxGetDevice(device: *mut CUdevice) -> CUresult;
}
extern "C" {
    pub fn cuCtxSynchronize() -> CUresult;
}
extern "C" {
    pub fn cuCtxGetApiVersion(ctx: CUcontext, version: *mut ::std::os::raw::c_uint) -> CUresult;
}
extern "C" {
    pub fn cuCtxGetStreamPriorityRange(
        leastPriority: *mut ::std::os::raw::c_int,
        greatestPriority: *mut ::std::os::raw::c_int,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleLoad(module: *mut CUmodule, fname: *const ::std::os::raw::c_char) -> CUresult;
}
extern "C" {
    pub fn cuModuleLoadData(
        module: *mut CUmodule,
        image: *const ::std::os::raw::c_void,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleLoadDataEx(
        module: *mut CUmodule,
        image: *const ::std::os::raw::c_void,
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleLoadFatBinary(
        module: *mut CUmodule,
        fatCubin: *const ::std::os::raw::c_void,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleUnload(hmod: CUmodule) -> CUresult;
}
extern "C" {
    pub fn cuModuleGetFunction(
        hfunc: *mut CUfunction,
        hmod: CUmodule,
        name: *const ::std::os::raw::c_char,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleGetTexRef(
        pTexRef: *mut CUtexref,
        hmod: CUmodule,
        name: *const ::std::os::raw::c_char,
    ) -> CUresult;
}
extern "C" {
    pub fn cuModuleGetSurfRef(
        pSurfRef: *mut CUsurfref,
        hmod: CUmodule,
        name: *const ::std::os::raw::c_char,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLinkCreate_v2(
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
        stateOut: *mut CUlinkState,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLinkAddData_v2(
        state: CUlinkState,
        type_: CUjitInputType,
        data: *mut ::std::os::raw::c_void,
        size: usize,
        name: *const ::std::os::raw::c_char,
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLinkAddFile_v2(
        state: CUlinkState,
        type_: CUjitInputType,
        path: *const ::std::os::raw::c_char,
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLinkComplete(
        state: CUlinkState,
        cubinOut: *mut *mut ::std::os::raw::c_void,
        sizeOut: *mut usize,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLinkDestroy(state: CUlinkState) -> CUresult;
}
extern "C" {
    pub fn cuMemsetD8_v2(dstDevice: CUdeviceptr, uc: ::std::os::raw::c_uchar, N: usize)
        -> CUresult;
}
extern "C" {
    pub fn cuMemsetD16_v2(
        dstDevice: CUdeviceptr,
        us: ::std::os::raw::c_ushort,
        N: usize,
    ) -> CUresult;
}
extern "C" {
    pub fn cuMemsetD32_v2(dstDevice: CUdeviceptr, ui: ::std::os::raw::c_uint, N: usize)
        -> CUresult;
}
extern "C" {
    pub fn cuMemsetD8Async(
        dstDevice: CUdeviceptr,
        uc: ::std::os::raw::c_uchar,
        N: usize,
        hStream: CUstream,
    ) -> CUresult;
}
extern "C" {
    pub fn cuMemsetD16Async(
        dstDevice: CUdeviceptr,
        us: ::std::os::raw::c_ushort,
        N: usize,
        hStream: CUstream,
    ) -> CUresult;
}
extern "C" {
    pub fn cuMemsetD32Async(
        dstDevice: CUdeviceptr,
        ui: ::std::os::raw::c_uint,
        N: usize,
        hStream: CUstream,
    ) -> CUresult;
}
extern "C" {
    pub fn cuPointerGetAttribute(
        data: *mut ::std::os::raw::c_void,
        attribute: CUpointer_attribute,
        ptr: CUdeviceptr,
    ) -> CUresult;
}
extern "C" {
    pub fn cuPointerSetAttribute(
        value: *const ::std::os::raw::c_void,
        attribute: CUpointer_attribute,
        ptr: CUdeviceptr,
    ) -> CUresult;
}
extern "C" {
    pub fn cuLaunchKernel(
        f: CUfunction,
        gridDimX: ::std::os::raw::c_uint,
        gridDimY: ::std::os::raw::c_uint,
        gridDimZ: ::std::os::raw::c_uint,
        blockDimX: ::std::os::raw::c_uint,
        blockDimY: ::std::os::raw::c_uint,
        blockDimZ: ::std::os::raw::c_uint,
        sharedMemBytes: ::std::os::raw::c_uint,
        hStream: CUstream,
        kernelParams: *mut *mut ::std::os::raw::c_void,
        extra: *mut *mut ::std::os::raw::c_void,
    ) -> CUresult;
}

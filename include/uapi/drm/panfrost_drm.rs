//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/panfrost_drm.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: MIT
//
// Copyright © 2014-2018 Broadcom
// Copyright © 2019 Collabora ltd.
//

pub const DRM_PANFROST_SUBMIT: c_uint = 0x00;
pub const DRM_PANFROST_WAIT_BO: c_uint = 0x01;
pub const DRM_PANFROST_CREATE_BO: c_uint = 0x02;
pub const DRM_PANFROST_MMAP_BO: c_uint = 0x03;
pub const DRM_PANFROST_GET_PARAM: c_uint = 0x04;
pub const DRM_PANFROST_GET_BO_OFFSET: c_uint = 0x05;
pub const DRM_PANFROST_PERFCNT_ENABLE: c_uint = 0x06;
pub const DRM_PANFROST_PERFCNT_DUMP: c_uint = 0x07;
pub const DRM_PANFROST_MADVISE: c_uint = 0x08;
pub const DRM_PANFROST_SET_LABEL_BO: c_uint = 0x09;
pub const DRM_PANFROST_JM_CTX_CREATE: c_uint = 0x0a;
pub const DRM_PANFROST_JM_CTX_DESTROY: c_uint = 0x0b;
pub const DRM_PANFROST_SYNC_BO: c_uint = 0x0c;
pub const DRM_PANFROST_QUERY_BO_INFO: c_uint = 0x0d;

//
// Unstable ioctl(s): only exposed when the unsafe unstable_ioctls module
// param is set to true.
// All these ioctl(s) are subject to deprecation, so please don't rely on
// them for anything but debugging purpose.
//

//
// struct drm_panfrost_submit - ioctl argument for submitting commands to the 3D
// engine.
//
// This asks the kernel to have the GPU execute a render command list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_submit {
//
// @jc: Address to GPU mapping of job descriptor
//
    pub jc: __u64,
//
// @in_syncs: An optional array of sync objects to wait on
// before starting this job.
//
    pub in_syncs: __u64,
//
// @in_sync_count: Number of sync objects to wait on before
// starting this job.
//
    pub in_sync_count: __u32,
//
// @out_sync: An optional sync object to place the completion fence in.
//
    pub out_sync: __u32,
//
// @bo_handles: Pointer to a u32 array of the BOs that are
// referenced by the job.
//
    pub bo_handles: __u64,
//
// @bo_handle_count: Number of BO handles passed in (size is
// that times 4).
//
    pub bo_handle_count: __u32,
//
// @requirements: A combination of PANFROST_JD_REQ_
//
    pub requirements: __u32,
//
// @jm_ctx_handle: JM context handle. Zero if you want to use the
// default context.
//
    pub jm_ctx_handle: __u32,
//
// @pad: Padding field. Must be zero.
//
    pub pad: __u32,
}

//
// struct drm_panfrost_wait_bo - ioctl argument for waiting for
// completion of the last DRM_PANFROST_SUBMIT on a BO.
//
// This is useful for cases where multiple processes might be
// rendering to a BO and you want to wait for all rendering to be
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_wait_bo {
//
// @handle: Handle for the object to wait for.
//
    pub handle: __u32,
//
// @pad: Padding, must be zero-filled.
//
    pub pad: __u32,
//
// @timeout_ns: absolute number of nanoseconds to wait.
//
    pub timeout_ns: __s64,
}

// Valid flags to pass to drm_panfrost_create_bo.
// PANFROST_BO_WB_MMAP can't be set if PANFROST_BO_HEAP is.
//
pub const PANFROST_BO_NOEXEC: c_int = 1;
pub const PANFROST_BO_HEAP: c_int = 2;
pub const PANFROST_BO_WB_MMAP: c_int = 4;
//
// struct drm_panfrost_create_bo - ioctl argument for creating Panfrost BOs.
//
// The flags argument is a bit mask of PANFROST_BO_* flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_create_bo {
//
// @size: size of shmem/BO area to create (bytes)
//
    pub size: __u32,
//
// @flags: see PANFROST_BO_* flags
//
    pub flags: __u32,
//
// @handle: Returned GEM handle for the BO.
//
    pub handle: __u32,
//
// @pad: Padding, must be zero-filled.
//
    pub pad: __u32,
//
// @offset: Returned offset for the BO in the GPU address space.
// This offset is private to the DRM fd and is valid for the
// lifetime of the GEM handle.
//
// This offset value will always be nonzero, since various HW
// units treat 0 specially.
//
    pub offset: __u64,
}

//
// struct drm_panfrost_mmap_bo - ioctl argument for mapping Panfrost BOs.
//
// This doesn't actually perform an mmap.  Instead, it returns the
// offset you need to use in an mmap on the DRM device node.  This
// means that tools like valgrind end up knowing about the mapped
// memory.
//
// There are currently no values for the flags argument, but it may be
// used in a future extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_mmap_bo {
//
// @handle: Handle for the object being mapped.
//
    pub handle: __u32,
//
// @flags: currently not used (should be zero)
//
    pub flags: __u32,
//
// @offset: offset into the drm node to use for subsequent mmap call.
//
    pub offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panfrost_param {
    DRM_PANFROST_PARAM_GPU_PROD_ID,
    DRM_PANFROST_PARAM_GPU_REVISION,
    DRM_PANFROST_PARAM_SHADER_PRESENT,
    DRM_PANFROST_PARAM_TILER_PRESENT,
    DRM_PANFROST_PARAM_L2_PRESENT,
    DRM_PANFROST_PARAM_STACK_PRESENT,
    DRM_PANFROST_PARAM_AS_PRESENT,
    DRM_PANFROST_PARAM_JS_PRESENT,
    DRM_PANFROST_PARAM_L2_FEATURES,
    DRM_PANFROST_PARAM_CORE_FEATURES,
    DRM_PANFROST_PARAM_TILER_FEATURES,
    DRM_PANFROST_PARAM_MEM_FEATURES,
    DRM_PANFROST_PARAM_MMU_FEATURES,
    DRM_PANFROST_PARAM_THREAD_FEATURES,
    DRM_PANFROST_PARAM_MAX_THREADS,
    DRM_PANFROST_PARAM_THREAD_MAX_WORKGROUP_SZ,
    DRM_PANFROST_PARAM_THREAD_MAX_BARRIER_SZ,
    DRM_PANFROST_PARAM_COHERENCY_FEATURES,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES0,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES1,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES2,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES3,
    DRM_PANFROST_PARAM_JS_FEATURES0,
    DRM_PANFROST_PARAM_JS_FEATURES1,
    DRM_PANFROST_PARAM_JS_FEATURES2,
    DRM_PANFROST_PARAM_JS_FEATURES3,
    DRM_PANFROST_PARAM_JS_FEATURES4,
    DRM_PANFROST_PARAM_JS_FEATURES5,
    DRM_PANFROST_PARAM_JS_FEATURES6,
    DRM_PANFROST_PARAM_JS_FEATURES7,
    DRM_PANFROST_PARAM_JS_FEATURES8,
    DRM_PANFROST_PARAM_JS_FEATURES9,
    DRM_PANFROST_PARAM_JS_FEATURES10,
    DRM_PANFROST_PARAM_JS_FEATURES11,
    DRM_PANFROST_PARAM_JS_FEATURES12,
    DRM_PANFROST_PARAM_JS_FEATURES13,
    DRM_PANFROST_PARAM_JS_FEATURES14,
    DRM_PANFROST_PARAM_JS_FEATURES15,
    DRM_PANFROST_PARAM_NR_CORE_GROUPS,
    DRM_PANFROST_PARAM_THREAD_TLS_ALLOC,
    DRM_PANFROST_PARAM_AFBC_FEATURES,
    DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP,
    DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP_FREQUENCY,
    DRM_PANFROST_PARAM_ALLOWED_JM_CTX_PRIORITIES,
    DRM_PANFROST_PARAM_SELECTED_COHERENCY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panfrost_gpu_coherency {
    DRM_PANFROST_GPU_COHERENCY_ACE_LITE = 0,
    DRM_PANFROST_GPU_COHERENCY_ACE = 1,
    DRM_PANFROST_GPU_COHERENCY_NONE = 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_get_param {
    pub param: __u32,
    pub pad: __u32,
    pub value: __u64,
}

//
// Returns the offset for the BO in the GPU address space for this DRM fd.
// This is the same value returned by drm_panfrost_create_bo, if that was called
// from this DRM fd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_get_bo_offset {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_perfcnt_enable {
    pub enable: __u32,
//
// On bifrost we have 2 sets of counters, this parameter defines the
// one to track.
//
    pub counterset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_perfcnt_dump {
    pub buf_ptr: __u64,
}

// madvise provides a way to tell the kernel in case a buffers contents
// can be discarded under memory pressure, which is useful for userspace
// bo cache where we want to optimistically hold on to buffer allocate
// and potential mmap, but allow the pages to be discarded under memory
// pressure.
//
// Typical usage would involve madvise(DONTNEED) when buffer enters BO
// cache, and madvise(WILLNEED) if trying to recycle buffer from BO cache.
// In the WILLNEED case, 'retained' indicates to userspace whether the
// backing pages still exist.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_madvise {
    pub /: *mut *mut __u32 handle; / in, GEM handle,
    pub /: *mut *mut __u32 madv; / in, PANFROST_MADV_x,
    pub /: *mut *mut __u32 retained; / out, whether backing store still exists,
}

//
// struct drm_panfrost_set_label_bo - ioctl argument for labelling Panfrost BOs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_set_label_bo {
//
// @handle: Handle of the buffer object to label.
//
    pub handle: __u32,
//
// @pad: Must be zero.
//
    pub pad: __u32,
//
// @label: User pointer to a NUL-terminated string
//
// Length cannot be greater than 4096.
// NULL is permitted and means clear the label.
//
    pub label: __u64,
}

// Valid flags to pass to drm_panfrost_bo_sync_op
pub const PANFROST_BO_SYNC_CPU_CACHE_FLUSH: c_int = 0;
pub const PANFROST_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE: c_int = 1;
//
// struct drm_panthor_bo_flush_map_op - BO map sync op
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_bo_sync_op {
// @handle: Handle of the buffer object to sync.
    pub handle: __u32,
// @type: Type of sync operation.
    pub type: __u32,
//
// @offset: Offset into the BO at which the sync range starts.
//
// This will be rounded down to the nearest cache line as needed.
//
    pub offset: __u32,
//
// @size: Size of the range to sync
//
// @size + @offset will be rounded up to the nearest cache line as
// needed.
//
    pub size: __u32,
}

//
// struct drm_panfrost_sync_bo - ioctl argument for syncing BO maps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_sync_bo {
// Array of struct drm_panfrost_bo_sync_op
    pub ops: __u64,
// Number of BO sync ops
    pub op_count: __u32,
    pub pad: __u32,
}

// BO comes from a different subsystem.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_query_bo_info {
// Handle of the object being queried.
    pub handle: __u32,
// Extra flags that are not coming from the BO_CREATE ioctl().
    pub extra_flags: __u32,
// Flags passed at creation time.
    pub create_flags: __u32,
// Will be zero on return.
    pub pad: __u32,
}

// Definitions for coredump decoding in user space
pub const PANFROSTDUMP_MAJOR: c_int = 1;
pub const PANFROSTDUMP_MINOR: c_int = 0;
pub const PANFROSTDUMP_MAGIC: c_uint = 0x464E4150 /* PANF */;
pub const PANFROSTDUMP_BUF_REG: c_int = 0;

//
// This structure is the native endianness of the dumping machine, tools can
// detect the endianness by looking at the value in 'magic'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_dump_object_header {
    pub magic: __u32,
    pub type: __u32,
    pub file_size: __u32,
    pub file_offset: __u32,
    pub jc: __u64,
    pub gpu_id: __u32,
    pub major: __u32,
    pub minor: __u32,
    pub nbos: __u64,
    pub reghdr: },
    pub valid: __u32,
    pub iova: __u64,
    pub data: [__u32; 2],
    pub bomap: },
//
// Force same size in case we want to expand the header
// with new fields and also keep it 512-byte aligned
//
    pub sizer: [__u32; 496],
}

// Registers object, an array of these
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_dump_registers {
    pub reg: __u32,
    pub value: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panfrost_jm_ctx_priority {
//
// @PANFROST_JM_CTX_PRIORITY_LOW: Low priority context.
//
    PANFROST_JM_CTX_PRIORITY_LOW = 0,

//
// @PANFROST_JM_CTX_PRIORITY_MEDIUM: Medium priority context.
//
    PANFROST_JM_CTX_PRIORITY_MEDIUM,

//
// @PANFROST_JM_CTX_PRIORITY_HIGH: High priority context.
//
// Requires CAP_SYS_NICE or DRM_MASTER.
//
    PANFROST_JM_CTX_PRIORITY_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_jm_ctx_create {
//
// @handle: Handle of the created JM context
//
    pub handle: __u32,
//
// @priority: Context priority (see enum drm_panfrost_jm_ctx_priority).
//
    pub priority: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_jm_ctx_destroy {
//
// @handle: Handle of the JM context to destroy.
//
// Must be a valid context handle returned by DRM_IOCTL_PANTHOR_JM_CTX_CREATE.
//
    pub handle: __u32,
//
// @pad: Padding field, must be zero.
//
    pub pad: __u32,
}


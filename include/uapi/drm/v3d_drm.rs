//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/v3d_drm.h
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


//
// Copyright © 2014-2018 Broadcom
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

pub const DRM_V3D_SUBMIT_CL: c_uint = 0x00;
pub const DRM_V3D_WAIT_BO: c_uint = 0x01;
pub const DRM_V3D_CREATE_BO: c_uint = 0x02;
pub const DRM_V3D_MMAP_BO: c_uint = 0x03;
pub const DRM_V3D_GET_PARAM: c_uint = 0x04;
pub const DRM_V3D_GET_BO_OFFSET: c_uint = 0x05;
pub const DRM_V3D_SUBMIT_TFU: c_uint = 0x06;
pub const DRM_V3D_SUBMIT_CSD: c_uint = 0x07;
pub const DRM_V3D_PERFMON_CREATE: c_uint = 0x08;
pub const DRM_V3D_PERFMON_DESTROY: c_uint = 0x09;
pub const DRM_V3D_PERFMON_GET_VALUES: c_uint = 0x0a;
pub const DRM_V3D_SUBMIT_CPU: c_uint = 0x0b;
pub const DRM_V3D_PERFMON_GET_COUNTER: c_uint = 0x0c;
pub const DRM_V3D_PERFMON_SET_GLOBAL: c_uint = 0x0d;

pub const DRM_V3D_SUBMIT_CL_FLUSH_CACHE: c_uint = 0x01;
pub const DRM_V3D_SUBMIT_EXTENSION: c_uint = 0x02;
// struct drm_v3d_extension - ioctl extensions
//
// Linked-list of generic extensions where the id identify which struct is
// pointed by ext_data. Therefore, DRM_V3D_EXT_ID_* is used on id to identify
// the extension type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_extension {
    pub next: __u64,
    pub id: __u32,
pub const DRM_V3D_EXT_ID_MULTI_SYNC: c_uint = 0x01;
pub const DRM_V3D_EXT_ID_CPU_INDIRECT_CSD: c_uint = 0x02;
pub const DRM_V3D_EXT_ID_CPU_TIMESTAMP_QUERY: c_uint = 0x03;
pub const DRM_V3D_EXT_ID_CPU_RESET_TIMESTAMP_QUERY: c_uint = 0x04;
pub const DRM_V3D_EXT_ID_CPU_COPY_TIMESTAMP_QUERY: c_uint = 0x05;
pub const DRM_V3D_EXT_ID_CPU_RESET_PERFORMANCE_QUERY: c_uint = 0x06;
pub const DRM_V3D_EXT_ID_CPU_COPY_PERFORMANCE_QUERY: c_uint = 0x07;
    pub /: *mut *mut __u32 flags; / mbz,
}

// struct drm_v3d_sem - wait/signal semaphore
//
// If binary semaphore, it only takes syncobj handle and ignores flags and
// point fields. Point is defined for timeline syncobj feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_sem {
    pub /: *mut *mut __u32 handle; / syncobj,
// rsv below, for future uses
    pub flags: __u32,
    pub /: *mut *mut __u64 point; / for timeline sem support,
    pub /: *mut *mut __u64 mbz[2]; / must be zero, rsv,
}

// Enum for each of the V3D queues.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v3d_queue {
    V3D_BIN,
    V3D_RENDER,
    V3D_TFU,
    V3D_CSD,
    V3D_CACHE_CLEAN,
    V3D_CPU,
}

//
// struct drm_v3d_multi_sync - ioctl extension to add support multiples
// syncobjs for commands submission.
//
// When an extension of DRM_V3D_EXT_ID_MULTI_SYNC id is defined, it points to
// this extension to define wait and signal dependencies, instead of single
// in/out sync entries on submitting commands. The field flags is used to
// determine the stage to set wait dependencies.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_multi_sync {
    pub base: drm_v3d_extension,
// Array of wait and signal semaphores
    pub in_syncs: __u64,
    pub out_syncs: __u64,
// Number of entries
    pub in_sync_count: __u32,
    pub out_sync_count: __u32,
// set the stage (v3d_queue) to sync
    pub wait_stage: __u32,
    pub /: *mut *mut __u32 pad; / mbz,
}

//
// struct drm_v3d_submit_cl - ioctl argument for submitting commands to the 3D
// engine.
//
// This asks the kernel to have the GPU execute an optional binner
// command list, and a render command list.
//
// The L1T, slice, L2C, L2T, and GCA caches will be flushed before
// each CL executes.  The VCD cache should be flushed (if necessary)
// by the submitted CLs.  The TLB writes are guaranteed to have been
// flushed by the time the render done IRQ happens, which is the
// trigger for out_sync.  Any dirtying of cachelines by the job (only
// possible using TMU writes) must be flushed by the caller using the
// DRM_V3D_SUBMIT_CL_FLUSH_CACHE_FLAG flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_cl {
// Pointer to the binner command list.
//
// This is the first set of commands executed, which runs the
// coordinate shader to determine where primitives land on the screen,
// then writes out the state updates and draw calls necessary per tile
// to the tile allocation BO.
//
// This BCL will block on any previous BCL submitted on the
// same FD, but not on any RCL or BCLs submitted by other
// clients -- that is left up to the submitter to control
// using in_sync_bcl if necessary.
//
    pub bcl_start: __u32,
// End address of the BCL (first byte after the BCL)
    pub bcl_end: __u32,
// Offset of the render command list.
//
// This is the second set of commands executed, which will either
// execute the tiles that have been set up by the BCL, or a fixed set
// of tiles (in the case of RCL-only blits).
//
// This RCL will block on this submit's BCL, and any previous
// RCL submitted on the same FD, but not on any RCL or BCLs
// submitted by other clients -- that is left up to the
// submitter to control using in_sync_rcl if necessary.
//
    pub rcl_start: __u32,
// End address of the RCL (first byte after the RCL)
    pub rcl_end: __u32,
// An optional sync object to wait on before starting the BCL.
    pub in_sync_bcl: __u32,
// An optional sync object to wait on before starting the RCL.
    pub in_sync_rcl: __u32,
// An optional sync object to place the completion fence in.
    pub out_sync: __u32,
// Offset of the tile alloc memory
//
// This is optional on V3D 3.3 (where the CL can set the value) but
// required on V3D 4.1.
//
    pub qma: __u32,
// Size of the tile alloc memory.
    pub qms: __u32,
// Offset of the tile state data array.
    pub qts: __u32,
// Pointer to a u32 array of the BOs that are referenced by the job.
//
    pub bo_handles: __u64,
// Number of BO handles passed in (size is that times 4).
    pub bo_handle_count: __u32,
// DRM_V3D_SUBMIT_* properties
    pub flags: __u32,
// ID of the perfmon to attach to this job. 0 means no perfmon.
    pub perfmon_id: __u32,
    pub pad: __u32,
// Pointer to an array of ioctl extensions
    pub extensions: __u64,
}

//
// struct drm_v3d_wait_bo - ioctl argument for waiting for
// completion of the last DRM_V3D_SUBMIT_CL on a BO.
//
// This is useful for cases where multiple processes might be
// rendering to a BO and you want to wait for all rendering to be
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_wait_bo {
    pub handle: __u32,
    pub pad: __u32,
    pub timeout_ns: __u64,
}

//
// struct drm_v3d_create_bo - ioctl argument for creating V3D BOs.
//
// There are currently no values for the flags argument, but it may be
// used in a future extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_create_bo {
    pub size: __u32,
    pub flags: __u32,
// Returned GEM handle for the BO.
    pub handle: __u32,
//
// Returned offset for the BO in the V3D address space.  This offset
// is private to the DRM fd and is valid for the lifetime of the GEM
// handle.
//
// This offset value will always be nonzero, since various HW
// units treat 0 specially.
//
    pub offset: __u32,
}

//
// struct drm_v3d_mmap_bo - ioctl argument for mapping V3D BOs.
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
pub struct drm_v3d_mmap_bo {
// Handle for the object being mapped.
    pub handle: __u32,
    pub flags: __u32,
// offset into the drm node to use for subsequent mmap call.
    pub offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_v3d_param {
    DRM_V3D_PARAM_V3D_UIFCFG,
    DRM_V3D_PARAM_V3D_HUB_IDENT1,
    DRM_V3D_PARAM_V3D_HUB_IDENT2,
    DRM_V3D_PARAM_V3D_HUB_IDENT3,
    DRM_V3D_PARAM_V3D_CORE0_IDENT0,
    DRM_V3D_PARAM_V3D_CORE0_IDENT1,
    DRM_V3D_PARAM_V3D_CORE0_IDENT2,
    DRM_V3D_PARAM_SUPPORTS_TFU,
    DRM_V3D_PARAM_SUPPORTS_CSD,
    DRM_V3D_PARAM_SUPPORTS_CACHE_FLUSH,
    DRM_V3D_PARAM_SUPPORTS_PERFMON,
    DRM_V3D_PARAM_SUPPORTS_MULTISYNC_EXT,
    DRM_V3D_PARAM_SUPPORTS_CPU_QUEUE,
    DRM_V3D_PARAM_MAX_PERF_COUNTERS,
    DRM_V3D_PARAM_SUPPORTS_SUPER_PAGES,
    DRM_V3D_PARAM_GLOBAL_RESET_COUNTER,
    DRM_V3D_PARAM_CONTEXT_RESET_COUNTER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_get_param {
    pub param: __u32,
    pub pad: __u32,
    pub value: __u64,
}

//
// Returns the offset for the BO in the V3D address space for this DRM fd.
// This is the same value returned by drm_v3d_create_bo, if that was called
// from this DRM fd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_get_bo_offset {
    pub handle: __u32,
    pub offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_tfu {
    pub icfg: __u32,
    pub iia: __u32,
    pub iis: __u32,
    pub ica: __u32,
    pub iua: __u32,
    pub ioa: __u32,
    pub ios: __u32,
    pub coef: [__u32; 4],
// First handle is the output BO, following are other inputs.
// 0 for unused.
//
    pub bo_handles: [__u32; 4],
// sync object to block on before running the TFU job.  Each TFU
// job will execute in the order submitted to its FD.  Synchronization
// against rendering jobs requires using sync objects.
//
    pub in_sync: __u32,
// Sync object to signal when the TFU job is done.
    pub out_sync: __u32,
    pub flags: __u32,
// Pointer to an array of ioctl extensions
    pub extensions: __u64,
    pub ioc: __u32,
    pub pad: __u32,
    pub v71: },
}

// Submits a compute shader for dispatch.  This job will block on any
// previous compute shaders submitted on this fd, and any other
// synchronization must be performed with in_sync/out_sync.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_csd {
    pub cfg: [__u32; 7],
    pub coef: [__u32; 4],
// Pointer to a u32 array of the BOs that are referenced by the job.
//
    pub bo_handles: __u64,
// Number of BO handles passed in (size is that times 4).
    pub bo_handle_count: __u32,
// sync object to block on before running the CSD job.  Each
// CSD job will execute in the order submitted to its FD.
// Synchronization against rendering/TFU jobs or CSD from
// other fds requires using sync objects.
//
    pub in_sync: __u32,
// Sync object to signal when the CSD job is done.
    pub out_sync: __u32,
// ID of the perfmon to attach to this job. 0 means no perfmon.
    pub perfmon_id: __u32,
// Pointer to an array of ioctl extensions
    pub extensions: __u64,
    pub flags: __u32,
    pub pad: __u32,
}

//
// struct drm_v3d_indirect_csd - ioctl extension for the CPU job to create an
// indirect CSD
//
// When an extension of DRM_V3D_EXT_ID_CPU_INDIRECT_CSD id is defined, it
// points to this extension to define a indirect CSD submission. It creates a
// CPU job linked to a CSD job. The CPU job waits for the indirect CSD
// dependencies and, once they are signaled, it updates the CSD job config
// before allowing the CSD job execution.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_indirect_csd {
    pub base: drm_v3d_extension,
// Indirect CSD
    pub submit: drm_v3d_submit_csd,
// Handle of the indirect BO, that should be also attached to the
// indirect CSD.
//
    pub indirect: __u32,
// Offset within the BO where the workgroup counts are stored
    pub offset: __u32,
// Workgroups size
    pub wg_size: __u32,
// Indices of the uniforms with the workgroup dispatch counts
// in the uniform stream. If the uniform rewrite is not needed,
// the offset must be 0xffffffff.
//
    pub wg_uniform_offsets: [__u32; 3],
}

//
// struct drm_v3d_timestamp_query - ioctl extension for the CPU job to calculate
// a timestamp query
//
// When an extension DRM_V3D_EXT_ID_TIMESTAMP_QUERY is defined, it points to
// this extension to define a timestamp query submission. This CPU job will
// calculate the timestamp query and update the query value within the
// timestamp BO. Moreover, it will signal the timestamp syncobj to indicate
// query availability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_timestamp_query {
    pub base: drm_v3d_extension,
// Array of queries' offsets within the timestamp BO for their value
    pub offsets: __u64,
// Array of timestamp's syncobjs to indicate its availability
    pub syncs: __u64,
// Number of queries
    pub count: __u32,
// mbz
    pub pad: __u32,
}

//
// struct drm_v3d_reset_timestamp_query - ioctl extension for the CPU job to
// reset timestamp queries
//
// When an extension DRM_V3D_EXT_ID_CPU_RESET_TIMESTAMP_QUERY is defined, it
// points to this extension to define a reset timestamp submission. This CPU
// job will reset the timestamp queries based on value offset of the first
// query. Moreover, it will reset the timestamp syncobj to reset query
// availability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_reset_timestamp_query {
    pub base: drm_v3d_extension,
// Array of timestamp's syncobjs to indicate its availability
    pub syncs: __u64,
// Offset of the first query within the timestamp BO for its value
    pub offset: __u32,
// Number of queries
    pub count: __u32,
}

//
// struct drm_v3d_copy_timestamp_query - ioctl extension for the CPU job to copy
// query results to a buffer
//
// When an extension DRM_V3D_EXT_ID_CPU_COPY_TIMESTAMP_QUERY is defined, it
// points to this extension to define a copy timestamp query submission. This
// CPU job will copy the timestamp queries results to a BO with the offset
// and stride defined in the extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_copy_timestamp_query {
    pub base: drm_v3d_extension,
// Define if should write to buffer using 64 or 32 bits
    pub do_64bit: __u8,
// Define if it can write to buffer even if the query is not available
    pub do_partial: __u8,
// Define if it should write availability bit to buffer
    pub availability_bit: __u8,
// mbz
    pub pad: __u8,
// Offset of the buffer in the BO
    pub offset: __u32,
// Stride of the buffer in the BO
    pub stride: __u32,
// Number of queries
    pub count: __u32,
// Array of queries' offsets within the timestamp BO for their value
    pub offsets: __u64,
// Array of timestamp's syncobjs to indicate its availability
    pub syncs: __u64,
}

//
// struct drm_v3d_reset_performance_query - ioctl extension for the CPU job to
// reset performance queries
//
// When an extension DRM_V3D_EXT_ID_CPU_RESET_PERFORMANCE_QUERY is defined, it
// points to this extension to define a reset performance submission. This CPU
// job will reset the performance queries by resetting the values of the
// performance monitors. Moreover, it will reset the syncobj to reset query
// availability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_reset_performance_query {
    pub base: drm_v3d_extension,
// Array of performance queries's syncobjs to indicate its availability
    pub syncs: __u64,
// Number of queries
    pub count: __u32,
// Number of performance monitors
    pub nperfmons: __u32,
// Array of u64 user-pointers that point to an array of kperfmon_ids
    pub kperfmon_ids: __u64,
}

//
// struct drm_v3d_copy_performance_query - ioctl extension for the CPU job to copy
// performance query results to a buffer
//
// When an extension DRM_V3D_EXT_ID_CPU_COPY_PERFORMANCE_QUERY is defined, it
// points to this extension to define a copy performance query submission. This
// CPU job will copy the performance queries results to a BO with the offset
// and stride defined in the extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_copy_performance_query {
    pub base: drm_v3d_extension,
// Define if should write to buffer using 64 or 32 bits
    pub do_64bit: __u8,
// Define if it can write to buffer even if the query is not available
    pub do_partial: __u8,
// Define if it should write availability bit to buffer
    pub availability_bit: __u8,
// mbz
    pub pad: __u8,
// Offset of the buffer in the BO
    pub offset: __u32,
// Stride of the buffer in the BO
    pub stride: __u32,
// Number of performance monitors
    pub nperfmons: __u32,
// Number of performance counters related to this query pool
    pub ncounters: __u32,
// Number of queries
    pub count: __u32,
// Array of performance queries's syncobjs to indicate its availability
    pub syncs: __u64,
// Array of u64 user-pointers that point to an array of kperfmon_ids
    pub kperfmon_ids: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_cpu {
// Pointer to a u32 array of the BOs that are referenced by the job.
//
// For DRM_V3D_EXT_ID_CPU_INDIRECT_CSD, it must contain only one BO,
// that contains the workgroup counts.
//
// For DRM_V3D_EXT_ID_TIMESTAMP_QUERY, it must contain only one BO,
// that will contain the timestamp.
//
// For DRM_V3D_EXT_ID_CPU_RESET_TIMESTAMP_QUERY, it must contain only
// one BO, that contains the timestamp.
//
// For DRM_V3D_EXT_ID_CPU_COPY_TIMESTAMP_QUERY, it must contain two
// BOs. The first is the BO where the timestamp queries will be written
// to. The second is the BO that contains the timestamp.
//
// For DRM_V3D_EXT_ID_CPU_RESET_PERFORMANCE_QUERY, it must contain no
// BOs.
//
// For DRM_V3D_EXT_ID_CPU_COPY_PERFORMANCE_QUERY, it must contain one
// BO, where the performance queries will be written.
//
    pub bo_handles: __u64,
// Number of BO handles passed in (size is that times 4).
    pub bo_handle_count: __u32,
    pub flags: __u32,
// Pointer to an array of ioctl extensions
    pub extensions: __u64,
}

// The performance counters index represented by this enum are deprecated and
// must no longer be used. These counters are only valid for V3D 4.2.
//
// In order to check for performance counter information,
// use DRM_IOCTL_V3D_PERFMON_GET_COUNTER.
//
// Don't use V3D_PERFCNT_NUM to retrieve the maximum number of performance
// counters. You should use DRM_IOCTL_V3D_GET_PARAM with the following
// parameter: DRM_V3D_PARAM_MAX_PERF_COUNTERS.
//
pub const DRM_V3D_MAX_PERF_COUNTERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_create {
    pub id: __u32,
    pub ncounters: __u32,
    pub counters: [__u8; DRM_V3D_MAX_PERF_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_destroy {
    pub id: __u32,
}

//
// Returns the values of the performance counters tracked by this
// perfmon (as an array of ncounters u64 values).
//
// No implicit synchronization is performed, so the user has to
// guarantee that any jobs using this perfmon have already been
// completed  (probably by blocking on the seqno returned by the
// last exec that used the perfmon).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_get_values {
    pub id: __u32,
    pub pad: __u32,
    pub values_ptr: __u64,
}

pub const DRM_V3D_PERFCNT_MAX_NAME: c_int = 64;
pub const DRM_V3D_PERFCNT_MAX_CATEGORY: c_int = 32;
pub const DRM_V3D_PERFCNT_MAX_DESCRIPTION: c_int = 256;
//
// struct drm_v3d_perfmon_get_counter - ioctl to get the description of a
// performance counter
//
// As userspace needs to retrieve information about the performance counters
// available, this IOCTL allows users to get information about a performance
// counter (name, category and description).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_get_counter {
//
// Counter ID
//
// Must be smaller than the maximum number of performance counters, which
// can be retrieve through DRM_V3D_PARAM_MAX_PERF_COUNTERS.
//
    pub counter: __u8,
// Name of the counter
    pub name: [__u8; DRM_V3D_PERFCNT_MAX_NAME],
// Category of the counter
    pub category: [__u8; DRM_V3D_PERFCNT_MAX_CATEGORY],
// Description of the counter
    pub description: [__u8; DRM_V3D_PERFCNT_MAX_DESCRIPTION],
// mbz
    pub reserved: [__u8; 7],
}

pub const DRM_V3D_PERFMON_CLEAR_GLOBAL: c_uint = 0x0001;
//
// struct drm_v3d_perfmon_set_global - ioctl to define a global performance
// monitor
//
// The global performance monitor will be used for all jobs. If a global
// performance monitor is defined, jobs with a self-defined performance
// monitor won't be allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_set_global {
    pub flags: __u32,
    pub id: __u32,
}


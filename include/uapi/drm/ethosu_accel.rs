//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/ethosu_accel.h
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
// Copyright (C) 2025 Arm, Ltd.

//
// DOC: IOCTL IDs
//
// enum drm_ethosu_ioctl_id - IOCTL IDs
//
// Place new ioctls at the end, don't re-order, don't replace or remove entries.
//
// These IDs are not meant to be used directly. Use the DRM_IOCTL_ETHOSU_xxx
// definitions instead.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_ethosu_ioctl_id {
// @DRM_ETHOSU_DEV_QUERY: Query device information.
    DRM_ETHOSU_DEV_QUERY = 0,

// @DRM_ETHOSU_BO_CREATE: Create a buffer object.
    DRM_ETHOSU_BO_CREATE,

// @DRM_ETHOSU_BO_WAIT: Wait on a buffer object's fence.
    DRM_ETHOSU_BO_WAIT,

//
// @DRM_ETHOSU_BO_MMAP_OFFSET: Get the file offset to pass to
// mmap to map a GEM object.
//
    DRM_ETHOSU_BO_MMAP_OFFSET,

//
// @DRM_ETHOSU_CMDSTREAM_BO_CREATE: Create a command stream buffer
// object.
//
    DRM_ETHOSU_CMDSTREAM_BO_CREATE,

// @DRM_ETHOSU_SUBMIT: Submit a job and BOs to run.
    DRM_ETHOSU_SUBMIT,

    DRM_ETHOSU_PERFMON_CREATE,
    DRM_ETHOSU_PERFMON_DESTROY,
    DRM_ETHOSU_PERFMON_GET_VALUES,
    DRM_ETHOSU_PERFMON_SET_GLOBAL,
}

//
// DOC: IOCTL arguments
//
// enum drm_ethosu_dev_query_type - Query type
//
// Place new types at the end, don't re-order, don't remove or replace.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_ethosu_dev_query_type {
// @DRM_ETHOSU_DEV_QUERY_NPU_INFO: Query NPU information.
    DRM_ETHOSU_DEV_QUERY_NPU_INFO = 0,
}

//
// struct drm_ethosu_gpu_info - NPU information
//
// Structure grouping all queryable information relating to the NPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_npu_info {
// @id : NPU ID.
    pub id: __u32,

// @gpu_rev: GPU revision.
    pub config: __u32,
    pub sram_size: __u32,
    pub pmu_counters: __u32,
}

//
// struct drm_ethosu_dev_query - Arguments passed to DRM_ETHOSU_IOCTL_DEV_QUERY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_dev_query {
// @type: the query type (see drm_ethosu_dev_query_type).
    pub type: __u32,
//
// @size: size of the type being queried.
//
// If pointer is NULL, size is updated by the driver to provide the
// output structure size. If pointer is not NULL, the driver will
// only copy min(size, actual_structure_size) bytes to the pointer,
// and update the size accordingly. This allows us to extend query
// types without breaking userspace.
//
    pub size: __u32,
//
// @pointer: user pointer to a query type struct.
//
// Pointer can be NULL, in which case, nothing is copied, but the
// actual structure size is returned. If not NULL, it must point to
// a location that's large enough to hold size bytes.
//
    pub pointer: __u64,
}

//
// enum drm_ethosu_bo_flags - Buffer object flags, passed at creation time.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_ethosu_bo_flags {
//
// @DRM_ETHOSU_BO_NO_MMAP: The buffer object will never be CPU-mapped
// in userspace.
//
    DRM_ETHOSU_BO_NO_MMAP = (1 << 0),
}

//
// struct drm_ethosu_bo_create - Arguments passed to DRM_IOCTL_ETHOSU_BO_CREATE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_bo_create {
//
// @size: Requested size for the object
//
// The (page-aligned) allocated size for the object will be returned.
//
    pub size: __u64,
//
// @flags: Flags. Must be a combination of drm_ethosu_bo_flags flags.
//
    pub flags: __u32,
//
// @handle: Returned handle for the object.
//
// Object handles are nonzero.
//
    pub handle: __u32,
}

//
// struct drm_ethosu_bo_mmap_offset - Arguments passed to DRM_IOCTL_ETHOSU_BO_MMAP_OFFSET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_bo_mmap_offset {
// @handle: Handle of the object we want an mmap offset for.
    pub handle: __u32,
// @pad: MBZ.
    pub pad: __u32,
// @offset: The fake offset to use for subsequent mmap calls.
    pub offset: __u64,
}

//
// struct drm_ethosu_wait_bo - ioctl argument for waiting for
// completion of the last DRM_ETHOSU_SUBMIT on a BO.
//
// This is useful for cases where multiple processes might be
// rendering to a BO and you want to wait for all rendering to be
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_bo_wait {
    pub handle: __u32,
    pub pad: __u32,
    pub /: *mut *mut __s64 timeout_ns; / absolute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_cmdstream_bo_create {
// Size of the data argument.
    pub size: __u32,
// Flags, currently must be 0.
    pub flags: __u32,
// Pointer to the data.
    pub data: __u64,
// Returned GEM handle for the BO.
    pub handle: __u32,
// Pad, must be 0.
    pub pad: __u32,
}

//
// struct drm_ethosu_job - A job to be run on the NPU
//
// The kernel will schedule the execution of this job taking into account its
// dependencies with other jobs. All tasks in the same job will be executed
// sequentially on the same core, to benefit from memory residency in SRAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_job {
// Input: BO handle for cmdstream.
    pub cmd_bo: __u32,
// Input: Amount of SRAM to use.
    pub sram_size: __u32,
pub const ETHOSU_MAX_REGIONS: c_int = 8;
// Input: Array of BO handles for each region.
    pub region_bo_handles: [__u32; ETHOSU_MAX_REGIONS],
}

//
// struct drm_ethosu_submit - ioctl argument for submitting commands to the NPU.
//
// The kernel will schedule the execution of these jobs in dependency order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_submit {
// Input: Pointer to an array of struct drm_ethosu_job.
    pub jobs: __u64,
// Input: Number of jobs passed in.
    pub job_count: __u32,
// Input: Id returned by DRM_ETHOSU_PERFMON_CREATE
    pub perfmon_id: __u32,
}

pub const DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_perfmon_create {
    pub id: __u32,
    pub ncounters: __u32,
    pub counters: [__u16; DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_perfmon_destroy {
    pub id: __u32,
    pub pad: __u32,
}

//
// Returns the values of the performance counters tracked by this
// perfmon (as an array of (ncounters + 1) u64 values).
//
// No implicit synchronization is performed, so the user has to
// guarantee that any jobs using this perfmon have already been
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_perfmon_get_values {
    pub id: __u32,
    pub pad: __u32,
    pub values_ptr: __u64,
}

pub const DRM_ETHOSU_PERFMON_CLEAR_GLOBAL: c_uint = 0x0001;
//
// struct drm_ethosu_perfmon_set_global - ioctl to define a global performance
// monitor
//
// The global performance monitor will be used for all jobs. If a global
// performance monitor is defined, jobs with a self-defined performance
// monitor won't be allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ethosu_perfmon_set_global {
    pub flags: __u32,
    pub id: __u32,
}

//
// DRM_IOCTL_ETHOSU() - Build a ethosu IOCTL number
// @__access: Access type. Must be R, W or RW.
// @__id: One of the DRM_ETHOSU_xxx id.
// @__type: Suffix of the type being passed to the IOCTL.
//
// Don't use this macro directly, use the DRM_IOCTL_ETHOSU_xxx
// values instead.
//
// Return: An IOCTL number to be passed to ioctl() from userspace.
//


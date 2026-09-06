//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/ivpu_accel.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2020-2025 Intel Corporation
//

pub const DRM_IVPU_GET_PARAM: c_uint = 0x00;
pub const DRM_IVPU_SET_PARAM: c_uint = 0x01;
pub const DRM_IVPU_BO_CREATE: c_uint = 0x02;
pub const DRM_IVPU_BO_INFO: c_uint = 0x03;
pub const DRM_IVPU_SUBMIT: c_uint = 0x05;
pub const DRM_IVPU_BO_WAIT: c_uint = 0x06;
pub const DRM_IVPU_METRIC_STREAMER_START: c_uint = 0x07;
pub const DRM_IVPU_METRIC_STREAMER_STOP: c_uint = 0x08;
pub const DRM_IVPU_METRIC_STREAMER_GET_DATA: c_uint = 0x09;
pub const DRM_IVPU_METRIC_STREAMER_GET_INFO: c_uint = 0x0a;
pub const DRM_IVPU_CMDQ_CREATE: c_uint = 0x0b;
pub const DRM_IVPU_CMDQ_DESTROY: c_uint = 0x0c;
pub const DRM_IVPU_CMDQ_SUBMIT: c_uint = 0x0d;
pub const DRM_IVPU_BO_CREATE_FROM_USERPTR: c_uint = 0x0e;

//
// DOC: contexts
//
// VPU contexts have private virtual address space, job queues and priority.
// Each context is identified by an unique ID. Context is created on open().
//
pub const DRM_IVPU_PARAM_DEVICE_ID: c_int = 0;
pub const DRM_IVPU_PARAM_DEVICE_REVISION: c_int = 1;
pub const DRM_IVPU_PARAM_PLATFORM_TYPE: c_int = 2;
pub const DRM_IVPU_PARAM_CORE_CLOCK_RATE: c_int = 3;
pub const DRM_IVPU_PARAM_NUM_CONTEXTS: c_int = 4;
pub const DRM_IVPU_PARAM_CONTEXT_BASE_ADDRESS: c_int = 5;

pub const DRM_IVPU_PARAM_CONTEXT_ID: c_int = 7;
pub const DRM_IVPU_PARAM_FW_API_VERSION: c_int = 8;
pub const DRM_IVPU_PARAM_ENGINE_HEARTBEAT: c_int = 9;
pub const DRM_IVPU_PARAM_UNIQUE_INFERENCE_ID: c_int = 10;
pub const DRM_IVPU_PARAM_TILE_CONFIG: c_int = 11;
pub const DRM_IVPU_PARAM_SKU: c_int = 12;
pub const DRM_IVPU_PARAM_CAPABILITIES: c_int = 13;
pub const DRM_IVPU_PARAM_PREEMPT_BUFFER_SIZE: c_int = 14;
pub const DRM_IVPU_PLATFORM_TYPE_SILICON: c_int = 0;
// Deprecated, use DRM_IVPU_JOB_PRIORITY
pub const DRM_IVPU_CONTEXT_PRIORITY_IDLE: c_int = 0;
pub const DRM_IVPU_CONTEXT_PRIORITY_NORMAL: c_int = 1;
pub const DRM_IVPU_CONTEXT_PRIORITY_FOCUS: c_int = 2;
pub const DRM_IVPU_CONTEXT_PRIORITY_REALTIME: c_int = 3;
pub const DRM_IVPU_JOB_PRIORITY_DEFAULT: c_int = 0;
pub const DRM_IVPU_JOB_PRIORITY_IDLE: c_int = 1;
pub const DRM_IVPU_JOB_PRIORITY_NORMAL: c_int = 2;
pub const DRM_IVPU_JOB_PRIORITY_FOCUS: c_int = 3;
pub const DRM_IVPU_JOB_PRIORITY_REALTIME: c_int = 4;
//
// DRM_IVPU_CAP_METRIC_STREAMER
//
// Metric streamer support. Provides sampling of various hardware performance
// metrics like DMA bandwidth and cache miss/hits. Can be used for profiling.
//
pub const DRM_IVPU_CAP_METRIC_STREAMER: c_int = 1;
//
// DRM_IVPU_CAP_DMA_MEMORY_RANGE
//
// Driver has capability to allocate separate memory range
// accessible by hardware DMA.
//
pub const DRM_IVPU_CAP_DMA_MEMORY_RANGE: c_int = 2;
//
// DRM_IVPU_CAP_MANAGE_CMDQ
//
// Driver supports explicit command queue operations like command queue create,
// command queue destroy and submit job on specific command queue.
//
pub const DRM_IVPU_CAP_MANAGE_CMDQ: c_int = 3;
//
// DRM_IVPU_CAP_BO_CREATE_FROM_USERPTR
//
// Driver supports creating buffer objects from user space memory pointers.
// This allows creating GEM buffers from existing user memory regions.
//
pub const DRM_IVPU_CAP_BO_CREATE_FROM_USERPTR: c_int = 4;
//
// struct drm_ivpu_param - Get/Set VPU parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_param {
//
// @param:
//
// Supported params:
//
// %DRM_IVPU_PARAM_DEVICE_ID:
// PCI Device ID of the VPU device (read-only)
//
// %DRM_IVPU_PARAM_DEVICE_REVISION:
// VPU device revision (read-only)
//
// %DRM_IVPU_PARAM_PLATFORM_TYPE:
// Returns %DRM_IVPU_PLATFORM_TYPE_SILICON on real hardware or device specific
// platform type when executing on a simulator or emulator (read-only)
//
// %DRM_IVPU_PARAM_CORE_CLOCK_RATE:
// Maximum frequency of the NPU data processing unit clock (read-only)
//
// %DRM_IVPU_PARAM_NUM_CONTEXTS:
// Maximum number of simultaneously existing contexts (read-only)
//
// %DRM_IVPU_PARAM_CONTEXT_BASE_ADDRESS:
// Lowest VPU virtual address available in the current context (read-only)
//
// %DRM_IVPU_PARAM_CONTEXT_ID:
// Current context ID, always greater than 0 (read-only)
//
// %DRM_IVPU_PARAM_FW_API_VERSION:
// Firmware API version array (read-only)
//
// %DRM_IVPU_PARAM_ENGINE_HEARTBEAT:
// Heartbeat value from an engine (read-only).
// Engine ID (i.e. DRM_IVPU_ENGINE_COMPUTE) is given via index.
//
// %DRM_IVPU_PARAM_UNIQUE_INFERENCE_ID:
// Device-unique inference ID (read-only)
//
// %DRM_IVPU_PARAM_TILE_CONFIG:
// VPU tile configuration  (read-only)
//
// %DRM_IVPU_PARAM_SKU:
// VPU SKU ID (read-only)
//
// %DRM_IVPU_PARAM_CAPABILITIES:
// Supported capabilities (read-only)
//
// %DRM_IVPU_PARAM_PREEMPT_BUFFER_SIZE:
// Size of the preemption buffer (read-only)
//
    pub param: __u32,
// @index: Index for params that have multiple instances
    pub index: __u32,
// @value: Param value
    pub value: __u64,
}

pub const DRM_IVPU_BO_SHAVE_MEM: c_uint = 0x00000001;

pub const DRM_IVPU_BO_MAPPABLE: c_uint = 0x00000002;
pub const DRM_IVPU_BO_DMA_MEM: c_uint = 0x00000004;
pub const DRM_IVPU_BO_READ_ONLY: c_uint = 0x00000008;
pub const DRM_IVPU_BO_CACHED: c_uint = 0x00000000;
pub const DRM_IVPU_BO_UNCACHED: c_uint = 0x00010000;
pub const DRM_IVPU_BO_WC: c_uint = 0x00020000;
pub const DRM_IVPU_BO_CACHE_MASK: c_uint = 0x00030000;

//
// struct drm_ivpu_bo_create - Create BO backed by SHMEM
//
// Create GEM buffer object allocated in SHMEM memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_bo_create {
// @size: The size in bytes of the allocated memory
    pub size: __u64,
//
// @flags:
//
// Supported flags:
//
// %DRM_IVPU_BO_HIGH_MEM:
//
// Allocate VPU address from >4GB range.
// Buffer object with vpu address >4GB can be always accessed by the
// VPU DMA engine, but some HW generation may not be able to access
// this memory from then firmware running on the VPU management processor.
// Suitable for input, output and some scratch buffers.
//
// %DRM_IVPU_BO_MAPPABLE:
//
// Buffer object can be mapped using mmap().
//
// %DRM_IVPU_BO_CACHED:
//
// Allocated BO will be cached on host side (WB) and snooped on the VPU side.
// This is the default caching mode.
//
// %DRM_IVPU_BO_UNCACHED:
//
// Not supported. Use DRM_IVPU_BO_WC instead.
//
// %DRM_IVPU_BO_WC:
//
// Allocated BO will use write combining buffer for writes but reads will be
// uncached.
//
    pub flags: __u32,
// @handle: Returned GEM object handle
    pub handle: __u32,
// @vpu_addr: Returned VPU virtual address
    pub vpu_addr: __u64,
}

//
// struct drm_ivpu_bo_create_from_userptr - Create dma-buf from user pointer
//
// Create a GEM buffer object from a user pointer to a memory region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_bo_create_from_userptr {
// @user_ptr: User pointer to memory region (must be page aligned)
    pub user_ptr: __u64,
// @size: Size of the memory region in bytes (must be page aligned)
    pub size: __u64,
//
// @flags:
//
// Supported flags:
//
// %DRM_IVPU_BO_HIGH_MEM:
//
// Allocate VPU address from >4GB range.
//
// %DRM_IVPU_BO_DMA_MEM:
//
// Allocate from DMA memory range accessible by hardware DMA.
//
// %DRM_IVPU_BO_READ_ONLY:
//
// Allocate as a read-only buffer object.
//
    pub flags: __u32,
// @handle: Returned GEM object handle
    pub handle: __u32,
// @vpu_addr: Returned VPU virtual address
    pub vpu_addr: __u64,
}

//
// struct drm_ivpu_bo_info - Query buffer object info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_bo_info {
// @handle: Handle of the queried BO
    pub handle: __u32,
// @flags: Returned flags used to create the BO
    pub flags: __u32,
// @vpu_addr: Returned VPU virtual address
    pub vpu_addr: __u64,
//
// @mmap_offset:
//
// Returned offset to be used in mmap(). 0 in case the BO is not mappable.
//
    pub mmap_offset: __u64,
// @size: Returned GEM object size, aligned to PAGE_SIZE
    pub size: __u64,
}

// drm_ivpu_submit engines
pub const DRM_IVPU_ENGINE_COMPUTE: c_int = 0;

//
// struct drm_ivpu_submit - Submit commands to the VPU
//
// Execute a single command buffer on a given VPU engine.
// Handles to all referenced buffer objects have to be provided in @buffers_ptr.
//
// User space may wait on job completion using %DRM_IVPU_BO_WAIT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_submit {
//
// @buffers_ptr:
//
// A pointer to an u32 array of GEM handles of the BOs required for this job.
// The number of elements in the array must be equal to the value given by @buffer_count.
//
// The first BO is the command buffer. The rest of array has to contain all
// BOs referenced from the command buffer.
//
    pub buffers_ptr: __u64,
// @buffer_count: Number of elements in the @buffers_ptr
    pub buffer_count: __u32,
//
// @engine: Select the engine this job should be executed on
//
// %DRM_IVPU_ENGINE_COMPUTE:
//
// Performs Deep Learning Neural Compute Inference Operations
//
    pub engine: __u32,
// @flags: Reserved for future use - must be zero
    pub flags: __u32,
//
// @commands_offset:
//
// Offset inside the first buffer in @buffers_ptr containing commands
// to be executed. The offset has to be 8-byte aligned.
//
    pub commands_offset: __u32,
//
// @priority:
//
// Priority to be set for related job command queue, can be one of the following:
// %DRM_IVPU_JOB_PRIORITY_DEFAULT
// %DRM_IVPU_JOB_PRIORITY_IDLE
// %DRM_IVPU_JOB_PRIORITY_NORMAL
// %DRM_IVPU_JOB_PRIORITY_FOCUS
// %DRM_IVPU_JOB_PRIORITY_REALTIME
//
    pub priority: __u32,
}

//
// struct drm_ivpu_cmdq_submit - Submit commands to the VPU using explicit command queue
//
// Execute a single command buffer on a given command queue.
// Handles to all referenced buffer objects have to be provided in @buffers_ptr.
//
// User space may wait on job completion using %DRM_IVPU_BO_WAIT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_cmdq_submit {
//
// @buffers_ptr:
//
// A pointer to an u32 array of GEM handles of the BOs required for this job.
// The number of elements in the array must be equal to the value given by @buffer_count.
//
// The first BO is the command buffer. The rest of array has to contain all
// BOs referenced from the command buffer.
//
    pub buffers_ptr: __u64,
// @buffer_count: Number of elements in the @buffers_ptr
    pub buffer_count: __u32,
// @cmdq_id: ID for the command queue where job will be submitted
    pub cmdq_id: __u32,
// @flags: Reserved for future use - must be zero
    pub flags: __u32,
//
// @commands_offset:
//
// Offset inside the first buffer in @buffers_ptr containing commands
// to be executed. The offset has to be 8-byte aligned.
//
    pub commands_offset: __u32,
//
// @preempt_buffer_index:
//
// Index of the preemption buffer in the buffers_ptr array.
//
    pub preempt_buffer_index: __u32,
    pub reserved: __u32,
}

// drm_ivpu_bo_wait job status codes
pub const DRM_IVPU_JOB_STATUS_SUCCESS: c_int = 0;
pub const DRM_IVPU_JOB_STATUS_ABORTED: c_int = 256;
//
// struct drm_ivpu_bo_wait - Wait for BO to become inactive
//
// Blocks until a given buffer object becomes inactive.
// With @timeout_ms set to 0 returns immediately.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_bo_wait {
// @handle: Handle to the buffer object to be waited on
    pub handle: __u32,
// @flags: Reserved for future use - must be zero
    pub flags: __u32,
// @timeout_ns: Absolute timeout in nanoseconds (may be zero)
    pub timeout_ns: __s64,
//
// @job_status:
//
// Job status code which is updated after the job is completed.
// &DRM_IVPU_JOB_STATUS_SUCCESS or device specific error otherwise.
// Valid only if @handle points to a command buffer.
//
    pub job_status: __u32,
// @pad: Padding - must be zero
    pub pad: __u32,
}

//
// struct drm_ivpu_metric_streamer_start - Start collecting metric data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_metric_streamer_start {
// @metric_group_mask: Indicates metric streamer instance
    pub metric_group_mask: __u64,
// @sampling_period_ns: Sampling period in nanoseconds
    pub sampling_period_ns: __u64,
//
// @read_period_samples:
//
// Number of samples after which user space will try to read the data.
// Reading the data after significantly longer period may cause data loss.
//
    pub read_period_samples: __u32,
// @sample_size: Returned size of a single sample in bytes
    pub sample_size: __u32,
// @max_data_size: Returned max @data_size from %DRM_IOCTL_IVPU_METRIC_STREAMER_GET_DATA
    pub max_data_size: __u32,
}

//
// struct drm_ivpu_metric_streamer_get_data - Copy collected metric data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_metric_streamer_get_data {
// @metric_group_mask: Indicates metric streamer instance
    pub metric_group_mask: __u64,
// @buffer_ptr: A pointer to a destination for the copied data
    pub buffer_ptr: __u64,
// @buffer_size: Size of the destination buffer
    pub buffer_size: __u64,
//
// @data_size: Returned size of copied metric data
//
// If the @buffer_size is zero, returns the amount of data ready to be copied.
//
    pub data_size: __u64,
}

// Command queue flags
pub const DRM_IVPU_CMDQ_FLAG_TURBO: c_uint = 0x00000001;
//
// struct drm_ivpu_cmdq_create - Create command queue for job submission
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_cmdq_create {
// @cmdq_id: Returned ID of created command queue
    pub cmdq_id: __u32,
//
// @priority:
//
// Priority to be set for related job command queue, can be one of the following:
// %DRM_IVPU_JOB_PRIORITY_DEFAULT
// %DRM_IVPU_JOB_PRIORITY_IDLE
// %DRM_IVPU_JOB_PRIORITY_NORMAL
// %DRM_IVPU_JOB_PRIORITY_FOCUS
// %DRM_IVPU_JOB_PRIORITY_REALTIME
//
    pub priority: __u32,
//
// @flags:
//
// Supported flags:
//
// %DRM_IVPU_CMDQ_FLAG_TURBO
//
// Enable low-latency mode for the command queue. The NPU will maximize performance
// when executing jobs from such queue at the cost of increased power usage.
//
    pub flags: __u32,
}

//
// struct drm_ivpu_cmdq_destroy - Destroy a command queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_cmdq_destroy {
// @cmdq_id: ID of command queue to destroy
    pub cmdq_id: __u32,
}

//
// struct drm_ivpu_metric_streamer_stop - Stop collecting metric data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ivpu_metric_streamer_stop {
// @metric_group_mask: Indicates metric streamer instance
    pub metric_group_mask: __u64,
}


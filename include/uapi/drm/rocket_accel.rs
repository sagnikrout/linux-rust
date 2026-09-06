//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/rocket_accel.h
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
// Copyright © 2024 Tomeu Vizoso
//

pub const DRM_ROCKET_CREATE_BO: c_uint = 0x00;
pub const DRM_ROCKET_SUBMIT: c_uint = 0x01;
pub const DRM_ROCKET_PREP_BO: c_uint = 0x02;
pub const DRM_ROCKET_FINI_BO: c_uint = 0x03;

//
// struct drm_rocket_create_bo - ioctl argument for creating Rocket BOs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_create_bo {
//
// @size: Input: Size of the requested BO.
//
    pub size: __u32,
//
// @handle: Output: GEM handle for the BO.
//
    pub handle: __u32,
//
// @dma_address: Output: DMA address for the BO in the NPU address
// space.  This address is private to the DRM fd and is valid for
// the lifetime of the GEM handle.
//
    pub dma_address: __u64,
//
// @offset: Output: Offset into the drm node to use for subsequent
// mmap call.
//
    pub offset: __u64,
}

//
// struct drm_rocket_prep_bo - ioctl argument for starting CPU ownership of the BO.
//
// Takes care of waiting for any NPU jobs that might still use the NPU and performs cache
// synchronization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_prep_bo {
//
// @handle: Input: GEM handle of the buffer object.
//
    pub handle: __u32,
//
// @reserved: Reserved, must be zero.
//
    pub reserved: __u32,
//
// @timeout_ns: Input: Amount of time to wait for NPU jobs.
//
    pub timeout_ns: __s64,
}

//
// struct drm_rocket_fini_bo - ioctl argument for finishing CPU ownership of the BO.
//
// Synchronize caches for NPU access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_fini_bo {
//
// @handle: Input: GEM handle of the buffer object.
//
    pub handle: __u32,
//
// @reserved: Reserved, must be zero.
//
    pub reserved: __u32,
}

//
// struct drm_rocket_task - A task to be run on the NPU
//
// A task is the smallest unit of work that can be run on the NPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_task {
//
// @regcmd: Input: DMA address to NPU mapping of register command buffer
//
    pub regcmd: __u32,
//
// @regcmd_count: Input: Number of commands in the register command
// buffer
//
    pub regcmd_count: __u32,
}

//
// struct drm_rocket_job - A job to be run on the NPU
//
// The kernel will schedule the execution of this job taking into account its
// dependencies with other jobs. All tasks in the same job will be executed
// sequentially on the same core, to benefit from memory residency in SRAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_job {
//
// @tasks: Input: Pointer to an array of struct drm_rocket_task.
//
    pub tasks: __u64,
//
// @in_bo_handles: Input: Pointer to a u32 array of the BOs that
// are read by the job.
//
    pub in_bo_handles: __u64,
//
// @out_bo_handles: Input: Pointer to a u32 array of the BOs that
// are written to by the job.
//
    pub out_bo_handles: __u64,
//
// @task_count: Input: Number of tasks passed in.
//
    pub task_count: __u32,
//
// @task_struct_size: Input: Size in bytes of the structs in the
// @tasks field.
//
    pub task_struct_size: __u32,
//
// @in_bo_handle_count: Input: Number of input BO handles passed in
// (size is that times 4).
//
    pub in_bo_handle_count: __u32,
//
// @out_bo_handle_count: Input: Number of output BO handles passed in
// (size is that times 4).
//
    pub out_bo_handle_count: __u32,
}

//
// struct drm_rocket_submit - ioctl argument for submitting commands to the NPU.
//
// The kernel will schedule the execution of these jobs in dependency order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rocket_submit {
//
// @jobs: Input: Pointer to an array of struct drm_rocket_job.
//
    pub jobs: __u64,
//
// @job_count: Input: Number of jobs passed in.
//
    pub job_count: __u32,
//
// @job_struct_size: Input: Size in bytes of the structs in the
// @jobs field.
//
    pub job_struct_size: __u32,
//
// @reserved: Reserved, must be zero.
//
    pub reserved: __u64,
}


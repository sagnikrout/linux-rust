//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_scheduler_abi.h
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
// Copyright © 2025 Intel Corporation
//

//
// Generic defines required for registration with and submissions to the GuC
// scheduler. Includes engine class/instance defines and context attributes
// (id, priority, etc)
//
// Engine classes/instances
pub const GUC_RENDER_CLASS: c_int = 0;
pub const GUC_VIDEO_CLASS: c_int = 1;
pub const GUC_VIDEOENHANCE_CLASS: c_int = 2;
pub const GUC_BLITTER_CLASS: c_int = 3;
pub const GUC_COMPUTE_CLASS: c_int = 4;
pub const GUC_GSC_OTHER_CLASS: c_int = 5;
pub const GUC_PAGING_CLASS: c_int = 6;

pub const GUC_MAX_ENGINE_CLASSES: c_int = 16;
pub const GUC_MAX_INSTANCES_PER_CLASS: c_int = 32;
// context priority values
pub const GUC_CLIENT_PRIORITY_KMD_HIGH: c_int = 0;
pub const GUC_CLIENT_PRIORITY_HIGH: c_int = 1;
pub const GUC_CLIENT_PRIORITY_KMD_NORMAL: c_int = 2;
pub const GUC_CLIENT_PRIORITY_NORMAL: c_int = 3;
pub const GUC_CLIENT_PRIORITY_NUM: c_int = 4;
// Context registration
pub const GUC_ID_MAX: c_int = 65535;
pub const GUC_ID_UNKNOWN: c_uint = 0xffffffff;

pub const GUC_CONTEXT_NORMAL: c_int = 0;
pub const GUC_CONTEXT_COMPRESSION_SAVE: c_int = 1;
pub const GUC_CONTEXT_COMPRESSION_RESTORE: c_int = 2;

// context enable/disable
pub const GUC_CONTEXT_DISABLE: c_int = 0;
pub const GUC_CONTEXT_ENABLE: c_int = 1;
// scheduler groups
pub const GUC_MAX_SCHED_GROUPS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_sched_group {
    pub engines: [u32; GUC_MAX_ENGINE_CLASSES],
    pub __packed: },

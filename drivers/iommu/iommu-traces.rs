//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/iommu-traces.c
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


// SPDX-License-Identifier: GPL-2.0
//
// iommu trace points
//
// Copyright (C) 2013 Shuah Khan <shuah.kh@samsung.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

// iommu_group_event
    EXPORT_TRACEPOINT_SYMBOL_GPL(add_device_to_group);
    EXPORT_TRACEPOINT_SYMBOL_GPL(remove_device_from_group);
// iommu_device_event
    EXPORT_TRACEPOINT_SYMBOL_GPL(attach_device_to_domain);
// iommu_map_unmap
    EXPORT_TRACEPOINT_SYMBOL_GPL(map);
    EXPORT_TRACEPOINT_SYMBOL_GPL(unmap);
// iommu_error
    EXPORT_TRACEPOINT_SYMBOL_GPL(io_page_fault);

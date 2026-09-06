//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_mmio_types.h
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
// Copyright © 2022-2026 Intel Corporation
//

//
// struct xe_mmio - register mmio structure
//
// Represents an MMIO region that the CPU may use to access registers.  A
// region may share its IO map with other regions (e.g., all GTs within a
// tile share the same map with their parent tile, but represent different
// subregions of the overall IO space).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_mmio {
// @tile: Backpointer to tile, used for tracing
    pub tile: *mut xe_tile,
// @regs: Map used to access registers.
    pub regs: *mut void __iomem,
//
// @sriov_vf_gt: Backpointer to GT.
//
// This pointer is only set for GT MMIO regions and only when running
// as an SRIOV VF structure
//
    pub sriov_vf_gt: *mut xe_gt,
//
// @regs_size: Length of the register region within the map.
//
// The size of the iomap set in *regs is generally larger than the
// register mmio space since it includes unused regions and/or
// non-register regions such as the GGTT PTEs.
//
    pub regs_size: usize,
// @adj_limit: adjust MMIO address if address is below this value
    pub adj_limit: u32,
// @adj_offset: offset to add to MMIO address when adjusting
    pub adj_offset: u32,
}

//
// struct xe_mmio_range - register range structure
//
// @start: first register offset in the range.
// @end: last register offset in the range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_mmio_range {
    pub start: u32,
    pub end: u32,
}

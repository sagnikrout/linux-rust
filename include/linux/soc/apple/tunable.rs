//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/apple/tunable.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple Silicon hardware tunable support
//
// Each tunable is a list with each entry containing a offset into the MMIO
// region, a mask of bits to be cleared and a set of bits to be set. These
// tunables are passed along by the previous boot stages and vary from device
// to device such that they cannot be hardcoded in the individual drivers.
//
// Copyright (C) The Asahi Linux Contributors
//

//
// Struct to store an Apple Silicon hardware tunable.
//
// Each tunable is a list with each entry containing a offset into the MMIO
// region, a mask of bits to be cleared and a set of bits to be set. These
// tunables are passed along by the previous boot stages and vary from device
// to device such that they cannot be hardcoded in the individual drivers.
//
// @param sz Number of [offset, mask, value] tuples stored in values.
// @param values [offset, mask, value] array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_tunable {
    pub sz: usize,
    pub offset: u32,
    pub mask: u32,
    pub value: u32,
    pub __counted_by(sz): } values[],
}

//
// Parse an array of hardware tunables from the device tree.
//
// @dev: Device node used for devm_kzalloc internally.
// @np: Device node which contains the tunable array.
// @name: Name of the device tree property which contains the tunables.
// @res: Resource to which the tunables will be applied, used for bound checking
//
// @return: devres allocated struct on success or PTR_ERR on failure.
//
// Apply a previously loaded hardware tunable.
//
// @param regs: MMIO to which the tunable will be applied.
// @param tunable: Pointer to the tunable.
//
extern "C" {
    pub fn apple_tunable_apply(regs: *mut void __iomem, tunable: *mut apple_tunable);
}

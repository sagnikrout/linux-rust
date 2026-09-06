//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/intel/pinctrl-tangier.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel Tangier pinctrl functions
//
// Copyright (C) 2016-2023 Intel Corporation
//
// Authors: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Raag Jadav <raag.jadav@intel.com>
//

pub const TNG_FAMILY_NR: c_int = 64;
pub const TNG_FAMILY_LEN: c_uint = 0x400;
//
// struct tng_family - Tangier pin family description
// @barno: MMIO BAR number where registers for this family reside
// @pin_base: Starting pin of pins in this family
// @npins: Number of pins in this family
// @protected: True if family is protected by access
// @regs: Family specific common registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_family {
    pub barno: c_uint,
    pub pin_base: c_uint,
    pub npins: usize,
    pub protected: bool,
    pub regs: *mut void __iomem,
}

//
// struct tng_pinctrl - Tangier pinctrl private structure
// @dev: Pointer to the device structure
// @lock: Lock to serialize register access
// @pctldesc: Pin controller description
// @pctldev: Pointer to the pin controller device
// @families: Array of families this pinctrl handles
// @nfamilies: Number of families in the array
// @functions: Array of functions
// @nfunctions: Number of functions in the array
// @groups: Array of pin groups
// @ngroups: Number of groups in the array
// @pins: Array of pins this pinctrl controls
// @npins: Number of pins in the array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_pinctrl {
    pub dev: *mut device,
    pub lock: raw_spinlock_t,
    pub pctldesc: pinctrl_desc,
    pub pctldev: *mut pinctrl_dev,
// Pin controller configuration
    pub families: *const tng_family,
    pub nfamilies: usize,
    pub functions: *const intel_function,
    pub nfunctions: usize,
    pub groups: *const intel_pingroup,
    pub ngroups: usize,
    pub pins: *const pinctrl_pin_desc,
    pub npins: usize,
}

extern "C" {
    pub fn devm_tng_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}

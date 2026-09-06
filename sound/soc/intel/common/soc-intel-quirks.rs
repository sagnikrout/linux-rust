//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/common/soc-intel-quirks.h
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
// soc-intel-quirks.h - prototypes for quirk autodetection
//
// Copyright (c) 2019, Intel Corporation.
//

//
// List of systems which:
// 1. Use a non CR version of the Bay Trail SoC
// 2. Contain at least 6 interrupt resources so that the
// platform_get_resource(pdev, IORESOURCE_IRQ, 5) check below
// succeeds
// 3. Despite 1. and 2. still have their IPC IRQ at index 0 rather then 5
//
// This needs to be here so that it can be shared between the SST and
// SOF drivers. We rely on the compiler to optimize this out in files
// where soc_intel_is_byt_cr is not used.
//
// bits 26:27 mirror PMIC options
//
// Some devices detected as BYT-T have only a single IRQ listed,
// causing platform_get_irq with index 5 to return -ENXIO.
// The correct IRQ in this case is at index 0, as on BYT-CR.
//


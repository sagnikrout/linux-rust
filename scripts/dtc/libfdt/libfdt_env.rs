//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/libfdt/libfdt_env.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// libfdt - Flat Device Tree manipulation
// Copyright (C) 2006 David Gibson, IBM Corporation.
// Copyright 2012 Kim Phillips, Freescale Semiconductor.
//

// Macro flag: #define FDT_FORCE
// Macro flag: #define FDT_BITWISE

pub type fdt16_t = uint16_t FDT_BITWISE;
pub type fdt32_t = uint32_t FDT_BITWISE;
pub type fdt64_t = uint64_t FDT_BITWISE;


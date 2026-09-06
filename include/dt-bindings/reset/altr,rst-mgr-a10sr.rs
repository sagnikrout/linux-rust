//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/altr,rst-mgr-a10sr.h
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
// Copyright Intel Corporation (C) 2017. All Rights Reserved
//
// Reset binding definitions for Altera Arria10 MAX5 System Resource Chip
//
// Adapted from altr,rst-mgr-a10.h
//
// Peripheral PHY resets
pub const A10SR_RESET_ENET_HPS: c_int = 0;
pub const A10SR_RESET_PCIE: c_int = 1;
pub const A10SR_RESET_FILE: c_int = 2;
pub const A10SR_RESET_BQSPI: c_int = 3;
pub const A10SR_RESET_USB: c_int = 4;
pub const A10SR_RESET_NUM: c_int = 5;

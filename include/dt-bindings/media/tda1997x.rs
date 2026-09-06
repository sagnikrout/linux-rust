//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/media/tda1997x.h
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
// Copyright (C) 2017 Gateworks Corporation
//
// TDA19973 36bit Video Port control registers
pub const TDA1997X_VP36_35_32: c_int = 0;
pub const TDA1997X_VP36_31_28: c_int = 1;
pub const TDA1997X_VP36_27_24: c_int = 2;
pub const TDA1997X_VP36_23_20: c_int = 3;
pub const TDA1997X_VP36_19_16: c_int = 4;
pub const TDA1997X_VP36_15_12: c_int = 5;
pub const TDA1997X_VP36_11_08: c_int = 6;
pub const TDA1997X_VP36_07_04: c_int = 7;
pub const TDA1997X_VP36_03_00: c_int = 8;
// TDA19971 24bit Video Port control registers
pub const TDA1997X_VP24_V23_20: c_int = 0;
pub const TDA1997X_VP24_V19_16: c_int = 1;
pub const TDA1997X_VP24_V15_12: c_int = 3;
pub const TDA1997X_VP24_V11_08: c_int = 4;
pub const TDA1997X_VP24_V07_04: c_int = 6;
pub const TDA1997X_VP24_V03_00: c_int = 7;
// Pin groups
pub const TDA1997X_VP_OUT_EN: c_uint = 0x80	/* enable output group */;
pub const TDA1997X_VP_HIZ: c_uint = 0x40	/* hi-Z output group when not used */;
pub const TDA1997X_VP_SWP: c_uint = 0x10	/* pin-swap output group */;

// pinswapped groups

// Audio bus DAI format

// Audio bus channel layout

// Audio bus clock
pub const TDA1997X_ACLK_16FS: c_int = 0;
pub const TDA1997X_ACLK_32FS: c_int = 1;
pub const TDA1997X_ACLK_64FS: c_int = 2;
pub const TDA1997X_ACLK_128FS: c_int = 3;
pub const TDA1997X_ACLK_256FS: c_int = 4;
pub const TDA1997X_ACLK_512FS: c_int = 5;

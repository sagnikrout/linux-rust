//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ks0127.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Video Capture Driver ( Video for Linux 1/2 )
// for the Matrox Marvel G200,G400 and Rainbow Runner-G series
//
// This module is an interface to the KS0127 video decoder chip.
//
// Copyright (C) 1999  Ryan Drake <stiletto@mediaone.net>
//
// input channels
pub const KS_INPUT_COMPOSITE_1: c_int = 0;
pub const KS_INPUT_COMPOSITE_2: c_int = 1;
pub const KS_INPUT_COMPOSITE_3: c_int = 2;
pub const KS_INPUT_COMPOSITE_4: c_int = 4;
pub const KS_INPUT_COMPOSITE_5: c_int = 5;
pub const KS_INPUT_COMPOSITE_6: c_int = 6;
pub const KS_INPUT_SVIDEO_1: c_int = 8;
pub const KS_INPUT_SVIDEO_2: c_int = 9;
pub const KS_INPUT_SVIDEO_3: c_int = 10;
pub const KS_INPUT_YUV656: c_int = 15;
pub const KS_INPUT_COUNT: c_int = 10;
// output channels
pub const KS_OUTPUT_YUV656E: c_int = 0;
pub const KS_OUTPUT_EXV: c_int = 1;
// video standards


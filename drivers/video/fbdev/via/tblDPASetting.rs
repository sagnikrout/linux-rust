//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/tblDPASetting.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

pub const DPA_CLK_30M: c_int = 30000000;
pub const DPA_CLK_50M: c_int = 50000000;
pub const DPA_CLK_70M: c_int = 70000000;
pub const DPA_CLK_100M: c_int = 100000000;
pub const DPA_CLK_150M: c_int = 150000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DPA_RANGE {
    DPA_CLK_RANGE_30M,
    DPA_CLK_RANGE_30_50M,
    DPA_CLK_RANGE_50_70M,
    DPA_CLK_RANGE_70_100M,
    DPA_CLK_RANGE_100_150M,
    DPA_CLK_RANGE_150M
}

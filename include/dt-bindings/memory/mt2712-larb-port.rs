//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mt2712-larb-port.h
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
// Copyright (c) 2017 MediaTek Inc.
// Author: Yong Wu <yong.wu@mediatek.com>
//

pub const M4U_LARB0_ID: c_int = 0;
pub const M4U_LARB1_ID: c_int = 1;
pub const M4U_LARB2_ID: c_int = 2;
pub const M4U_LARB3_ID: c_int = 3;
pub const M4U_LARB4_ID: c_int = 4;
pub const M4U_LARB5_ID: c_int = 5;
pub const M4U_LARB6_ID: c_int = 6;
pub const M4U_LARB7_ID: c_int = 7;
pub const M4U_LARB8_ID: c_int = 8;
pub const M4U_LARB9_ID: c_int = 9;
// larb0

// larb1

// larb2

// larb3

// larb4

// larb5

// larb6

// larb7

// larb8

// larb9


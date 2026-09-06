//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/mt65xx.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Hongzhou.Yang <hongzhou.yang@mediatek.com>
//

pub const MTK_PUPD_SET_R1R0_00: c_int = 100;
pub const MTK_PUPD_SET_R1R0_01: c_int = 101;
pub const MTK_PUPD_SET_R1R0_10: c_int = 102;
pub const MTK_PUPD_SET_R1R0_11: c_int = 103;
pub const MTK_PULL_SET_RSEL_000: c_int = 200;
pub const MTK_PULL_SET_RSEL_001: c_int = 201;
pub const MTK_PULL_SET_RSEL_010: c_int = 202;
pub const MTK_PULL_SET_RSEL_011: c_int = 203;
pub const MTK_PULL_SET_RSEL_100: c_int = 204;
pub const MTK_PULL_SET_RSEL_101: c_int = 205;
pub const MTK_PULL_SET_RSEL_110: c_int = 206;
pub const MTK_PULL_SET_RSEL_111: c_int = 207;
pub const MTK_DRIVE_2mA: c_int = 2;
pub const MTK_DRIVE_4mA: c_int = 4;
pub const MTK_DRIVE_6mA: c_int = 6;
pub const MTK_DRIVE_8mA: c_int = 8;
pub const MTK_DRIVE_10mA: c_int = 10;
pub const MTK_DRIVE_12mA: c_int = 12;
pub const MTK_DRIVE_14mA: c_int = 14;
pub const MTK_DRIVE_16mA: c_int = 16;
pub const MTK_DRIVE_20mA: c_int = 20;
pub const MTK_DRIVE_24mA: c_int = 24;
pub const MTK_DRIVE_28mA: c_int = 28;
pub const MTK_DRIVE_32mA: c_int = 32;

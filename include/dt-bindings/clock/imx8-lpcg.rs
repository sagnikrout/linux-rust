//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx8-lpcg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019-2020 NXP
// Dong Aisheng <aisheng.dong@nxp.com>
//
pub const IMX_LPCG_CLK_0: c_int = 0;
pub const IMX_LPCG_CLK_1: c_int = 4;
pub const IMX_LPCG_CLK_2: c_int = 8;
pub const IMX_LPCG_CLK_3: c_int = 12;
pub const IMX_LPCG_CLK_4: c_int = 16;
pub const IMX_LPCG_CLK_5: c_int = 20;
pub const IMX_LPCG_CLK_6: c_int = 24;
pub const IMX_LPCG_CLK_7: c_int = 28;

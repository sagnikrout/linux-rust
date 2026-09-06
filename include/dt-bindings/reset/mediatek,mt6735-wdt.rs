//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mediatek,mt6735-wdt.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
pub const MT6735_TOPRGU_MM_RST: c_int = 1;
pub const MT6735_TOPRGU_MFG_RST: c_int = 2;
pub const MT6735_TOPRGU_VENC_RST: c_int = 3;
pub const MT6735_TOPRGU_VDEC_RST: c_int = 4;
pub const MT6735_TOPRGU_IMG_RST: c_int = 5;
pub const MT6735_TOPRGU_MD_RST: c_int = 7;
pub const MT6735_TOPRGU_CONN_RST: c_int = 9;
pub const MT6735_TOPRGU_C2K_SW_RST: c_int = 14;
pub const MT6735_TOPRGU_C2K_RST: c_int = 15;
pub const MT6735_TOPRGU_RST_NUM: c_int = 9;

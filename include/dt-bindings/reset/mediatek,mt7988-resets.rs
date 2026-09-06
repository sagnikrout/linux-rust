//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mediatek,mt7988-resets.h
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
//
// Copyright (c) 2023 Daniel Golle <daniel@makrotopia.org>
// Author: Daniel Golle <daniel@makrotopia.org>
//
// ETHWARP resets
pub const MT7988_ETHWARP_RST_SWITCH: c_int = 0;
// INFRA resets
pub const MT7988_INFRA_RST0_PEXTP_MAC_SWRST: c_int = 0;
pub const MT7988_INFRA_RST1_THERM_CTRL_SWRST: c_int = 1;

//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/rk3188-cru.h
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
// Copyright (c) 2014 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//

// soft-reset indices
pub const SRST_PTM_CORE2: c_int = 0;
pub const SRST_PTM_CORE3: c_int = 1;
pub const SRST_CORE2: c_int = 5;
pub const SRST_CORE3: c_int = 6;
pub const SRST_CORE2_DBG: c_int = 10;
pub const SRST_CORE3_DBG: c_int = 11;
pub const SRST_TIMER2: c_int = 16;
pub const SRST_TIMER4: c_int = 23;
pub const SRST_I2S0: c_int = 24;
pub const SRST_TIMER5: c_int = 25;
pub const SRST_TIMER3: c_int = 29;
pub const SRST_TIMER6: c_int = 31;
pub const SRST_PTM3: c_int = 36;
pub const SRST_PTM3_ATB: c_int = 37;
pub const SRST_GPS: c_int = 67;
pub const SRST_HSICPHY: c_int = 75;
pub const SRST_TIMER: c_int = 78;
pub const SRST_PTM2: c_int = 92;
pub const SRST_CORE2_WDT: c_int = 94;
pub const SRST_CORE3_WDT: c_int = 95;
pub const SRST_PTM2_ATB: c_int = 111;
pub const SRST_HSIC: c_int = 117;
pub const SRST_CTI2: c_int = 118;
pub const SRST_CTI2_APB: c_int = 119;
pub const SRST_GPU_BRIDGE: c_int = 121;
pub const SRST_CTI3: c_int = 123;
pub const SRST_CTI3_APB: c_int = 124;
